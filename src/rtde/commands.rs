use std::io::Write;

use super::{
    as_bytes,
    data::DataType,
    types::{Header, Level, Message, PackageType, Payload, Protocol, Recipe, Version},
    Rtde,
};
use crate::prelude::*;

impl UniversalRobot {
    /// Retrieves the robot's major, minor, bugfix and build number.
    pub fn get_ur_version(&mut self) -> Result<Version> {
        let payload = Header::new(PackageType::URControlVersion, None);
        self.rtde
            .send(as_bytes(payload)?, PackageType::URControlVersion)
    }
    /// Send an info log message to the Robot.
    pub fn info(&mut self, message: &str, source: &str) -> Result<()> {
        self.rtde.send_message(message, source, Level::Info)
    }
    /// Send an error log message to the Robot.
    pub fn error(&mut self, message: &str, source: &str) -> Result<()> {
        self.rtde.send_message(message, source, Level::Error)
    }
    /// Send a warning log message to the Robot.
    pub fn warn(&mut self, message: &str, source: &str) -> Result<()> {
        self.rtde.send_message(message, source, Level::Warning)
    }
    /// Send an exception log message to the Robot.
    pub fn exception(&mut self, message: &str, source: &str) -> Result<()> {
        self.rtde.send_message(message, source, Level::Exception)
    }
}

impl Rtde {
    /// Request the robot to work with "protocol version".
    ///
    /// 1 (success) or 0 (failed). On success, the api should speak the specified protocol version and the Robot will answer in that version.
    pub fn set_protocol_version(&mut self, protocol: Protocol) -> Result<()> {
        let payload = Payload::new(PackageType::ProtocolVersion, protocol, None)?;
        if self.send::<bool>(as_bytes(payload)?, PackageType::ProtocolVersion)? {
            self.protocol = protocol;
            Ok(())
        } else {
            Err(Error::Static("protocol change rejected"))
        }
    }
    /// Send an exception, error, warning or info message.
    pub(crate) fn send_message(&mut self, message: &str, source: &str, level: Level) -> Result<()> {
        let message = Message::new(message, source, level);
        println!("{message:?}");
        let mut message_bytes = message.as_bytes()?;
        let header = Header::new(PackageType::Message, Some(3 + message_bytes.len() as u16));
        let mut bytes = as_bytes(header)?;
        bytes.append(&mut message_bytes);
        self.port.writer.write_all(&bytes)?;
        self.port.writer.flush()?;
        Ok(())
    }
    /// Request the robot to start sending output updates.
    ///
    /// This will fail if e.g. an output package has not been configured yet.
    pub fn start(&mut self) -> Result<()> {
        if self.output.is_empty() {
            return Err(Error::Static("must set up at least one rtde output recipe"));
        };
        let payload = Header::new(PackageType::Start, None);
        if self.send::<bool>(as_bytes(payload)?, PackageType::Start)? {
            Ok(())
        } else {
            Err(Error::Static("rtde play error"))
        }
    }
    /// Request the robot to pause sending output updates.
    ///
    /// Robot should always accept a pause command.
    pub fn pause(&mut self) -> Result<()> {
        let payload = Header::new(PackageType::Pause, None);
        if self.send::<bool>(as_bytes(payload)?, PackageType::Pause)? {
            Ok(())
        } else {
            Err(Error::Static("rtde pause error"))
        }
    }
    /// Setup the outputs recipe.
    ///
    /// At the moment the Robot only supports one output recipe
    /// and the output frequency is configurable.
    ///
    /// The frequency must be between 1 and 500 Hz and the output
    /// rate will be according to floor(500 / frequency).
    /// The package should contain all desired output variables.
    /// The variable names is a list of comma separated variable
    /// name strings.
    ///
    /// Returns the variable types in the same order as they were
    /// supplied in the request.
    pub fn setup_output(&mut self, recipe: &[&str], rate_hz: f64) -> Result<Recipe> {
        if !self.output.is_empty() {
            return Err(Error::Static("Cannot setup more than one output recipe"));
        }
        let mut rate_bytes = as_bytes(rate_hz)?;
        let mut recipe_bytes = recipe.join(",");
        recipe_bytes.push_str("\r\n");
        let mut recipe_bytes = recipe_bytes.as_bytes().to_vec();
        let header = Header::new(
            PackageType::SetupOutputs,
            Some(3 + (rate_bytes.len() + recipe_bytes.len()) as u16),
        );
        let mut bytes = as_bytes(header)?;
        bytes.append(&mut rate_bytes);
        bytes.append(&mut recipe_bytes);
        // write request
        self.port.writer.write_all(&bytes)?;
        self.port.writer.flush()?;
        // read response
        let response = self.read()?;
        match response.get_type() {
            PackageType::SetupOutputs => {
                let id: u8 = self.parse_bytes(&response.payload[..1])?;
                match std::str::from_utf8(&response.payload[1..]) {
                    Ok(resp) => {
                        self.output = resp.split(',').map(DataType::new).collect();
                        self.frequency = rate_hz;
                        Ok(Recipe::new(id, self.output.clone()))
                    }
                    Err(error) => Err(Error::Deserialization(error.to_string())),
                }
            }
            other => Err(Error::UnexpectedResponse(format!(
                "instead of setup outputs, found {other:?}",
            ))),
        }
    }
    /// Setup an input recipe.
    ///
    /// These are contracts set up by the remote to send custom variables to the Robot.
    /// They allow us to specify a list of data types and a corresponding Recipe ID (index).
    pub fn setup_input(&mut self, recipe: &[&str]) -> Result<Recipe> {
        let mut recipe_bytes = recipe.join(",");
        recipe_bytes.push_str("\r\n");
        let mut recipe_bytes = recipe_bytes.as_bytes().to_vec();
        let header = Header::new(
            PackageType::SetupInputs,
            Some(3 + recipe_bytes.len() as u16),
        );
        let mut bytes = as_bytes(header)?;
        bytes.append(&mut recipe_bytes);
        // write request
        self.port.writer.write_all(&bytes)?;
        self.port.writer.flush()?;
        // read response
        let response = self.read()?;
        match response.get_type() {
            PackageType::SetupInputs => {
                let id: u8 = self.parse_bytes(&response.payload[..1])?;
                if id == 0 {
                    return Err(Error::UnexpectedResponse(format!(
                        "input recipe {recipe:?} rejected"
                    )));
                }
                match std::str::from_utf8(&response.payload[1..]) {
                    Ok(resp) => Ok(Recipe::new(
                        id,
                        resp.split(',').map(DataType::new).collect(),
                    )),
                    Err(error) => Err(Error::Deserialization(error.to_string())),
                }
            }
            other => Err(Error::UnexpectedResponse(format!(
                "instead of setup inputs, found {other:?}",
            ))),
        }
    }
}
