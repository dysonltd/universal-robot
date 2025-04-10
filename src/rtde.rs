//! The Real-Time Data Exchange (RTDE) interface provides a way to synchronize
//! external applications with the UR controller over a standard TCP/IP connection,
//! without breaking any real-time properties of the UR controller. This functionality
//! is among others useful for interacting with fieldbus drivers (e.g. Ethernet/IP),
//! manipulating robot I/O and plotting robot status (e.g. robot trajectories).
//! The RTDE interface is by default available when the UR controller is running.
//!
//! When the synchronization loop is started, the RTDE interface sends the client the
//! requested data in the same order it was requested by the client. Furthermore the
//! client is expected to send updated inputs to the RTDE interface on a change of value.
//! The data synchronization uses serialized binary data.
#[cfg(test)]
mod test;

pub mod commands;
pub mod data;
pub mod types;

use serde::de::DeserializeOwned;
use serde::Serialize;
use types::Header;

use crate::physical::UrPort;
use crate::prelude::*;
use crate::rolling_buffer::RollingBuffer;

use self::data::DataType;
use self::types::{PackageType, Payload, Protocol};

use std::io::{ErrorKind, Read, Write};
use std::net::IpAddr;

/// Real-time Data Exchange
pub struct Rtde {
    port: UrPort,
    output: Vec<DataType>,
    frequency: f64,
    protocol: Protocol,
    messages: RollingBuffer<String>,
}

/// Convert this payload to a bytestream ready to send to the robot.
///
/// Doesn't work with strings.
pub fn as_bytes<T: Serialize>(payload: T) -> Result<Vec<u8>> {
    let config = bincode::config::legacy()
        .with_big_endian()
        .with_fixed_int_encoding();
    match bincode::serde::encode_to_vec(payload, config) {
        Err(error) => Err(Error::Serialization(error.to_string())),
        Ok(bytes) => Ok(bytes),
    }
}

impl Rtde {
    const RTDE_PORT: u16 = 30004;
    /// Initialize connection to the dashboard server port
    pub fn new(host: IpAddr, timeout: Option<Duration>) -> Result<Self> {
        let port = UrPort::new(host, timeout, Self::RTDE_PORT)?;
        let mut rtde = Rtde {
            port,
            output: Vec::new(),
            frequency: 50.0,
            protocol: Protocol::V2,
            messages: RollingBuffer::new(10),
        };
        rtde.set_protocol_version(Protocol::V2)?;
        Ok(rtde)
    }
    /// Convert from a bytestream to any type T required
    fn parse_bytes<T: DeserializeOwned>(&self, buf: &[u8]) -> Result<T> {
        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        match bincode::serde::decode_from_slice(buf, config) {
            Err(error) => Err(Error::Deserialization(error.to_string())),
            Ok((payload, _)) => Ok(payload),
        }
    }
    /// Read back bytes from the RTDE stream
    ///
    /// Expect the first 3 bytes to indicate the length of the full message
    /// and its type
    pub fn read(&mut self) -> Result<Payload<Vec<u8>>> {
        // read response (size & type)
        let mut header_buf = [0u8; 3];
        self.port.reader.read_exact(&mut header_buf)?;

        let payload_size: u16 = self.parse_bytes(&header_buf[..2])?;
        let package_type: PackageType = header_buf[2].try_into()?;

        // read payload with handling for fragmented data.
        let mut payload_buf = vec![0; (payload_size - 3) as usize];
        let mut bytes_read = 0;
        while bytes_read < payload_buf.len() {
            match self.port.reader.read(&mut payload_buf[bytes_read..]) {
                Ok(0) => return Err(Error::ConnectionLost),
                Ok(n) => bytes_read += n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Payload::new(package_type, payload_buf, Some(payload_size))
    }
    /// Write bytes to the RTDE stream.
    ///
    /// Data type of payload must match the input recipe specified by recipe_id
    /// or the Robot will reject this payload.
    ///
    /// No response.
    pub fn write<T: Serialize>(&mut self, payload: T, recipe_id: u8) -> Result<()> {
        let mut payload_bytes = as_bytes(recipe_id)?;
        payload_bytes.append(&mut as_bytes(payload)?);
        let header = Header::new(PackageType::Data, Some(3 + payload_bytes.len() as u16));
        let mut bytes = as_bytes(header)?;
        bytes.append(&mut payload_bytes);
        // write request
        self.port.writer.write_all(&bytes)?;
        self.port.writer.flush()?;
        Ok(())
    }
    /// Private boilerplate function to send a bytestream to the rtde with an expected response pattern.
    ///
    /// The buffer might have data in it that doesn't match what we're looking for, so need to parse it.
    fn send<Res>(&mut self, bytes: Vec<u8>, expect: PackageType) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.port.writer.write_all(&bytes)?;
        self.port.writer.flush()?;

        // at 500Hz this can take ~960 data reads before it flushes
        // so we'll assume 2000 is enough to have flushed through.
        let max_read_attempts = 2000;
        let mut responses_read = 0;

        while responses_read < max_read_attempts {
            let response = self.read()?;
            responses_read += 1;

            match response.get_type() {
                package_type if package_type == expect => {
                    log::debug!("recieved expected package after {responses_read} reads");
                    return self.parse_bytes(&response.payload);
                }
                PackageType::Message => {
                    let message = String::from_utf8_lossy(&response.payload).into_owned();
                    self.messages.add(message);
                }
                other => {
                    log::trace!("Received unwanted package type: {:?}", other);
                }
            }
        }
        Err(Error::MaxReads(expect))
    }
    /// End connection to the dashboard server port
    pub fn close(self) -> Result<Vec<String>> {
        self.port.close()?;
        let messages = self.messages.values();
        dbg!(&messages);
        Ok(messages)
    }
}

// send a message with an expected response type.  If the response doesn't match the expectation,
