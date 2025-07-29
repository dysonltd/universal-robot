//! Connection layer between the robot and this code API

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::SocketAddr;
use std::net::{IpAddr, TcpStream};

use crate::prelude::*;
use crate::Rtde;

/// Universal Robot Remote Control Interface
///
/// Allows query and control of many aspects of a Universal Robots arm.
/// Holds references to each used TCP Port used for different aspects of comms.
///
/// - Dashboard - basic commands
/// - RTDE (Real-time data exchange) - high speed custom data
pub struct UniversalRobot {
    pub dashboard: Dashboard,
    primary: UrPort,
    secondary: UrPort,
    pub rtde: Rtde,
}

impl UniversalRobot {
    const PRIMARY: u16 = 30001;
    const SECONDARY: u16 = 30002;
    /// Connect to universal robot tcp ports
    pub fn connect(address: IpAddr, timeout: Duration) -> Result<Self> {
        Ok(UniversalRobot {
            dashboard: Dashboard::new(address, Some(timeout))?,
            primary: UrPort::new(address, Some(timeout), Self::PRIMARY)?,
            secondary: UrPort::new(address, Some(timeout), Self::SECONDARY)?,
            rtde: Rtde::new(address, Some(timeout))?,
        })
    }
    /// Close connection to universal robot tcp ports
    pub fn close(self) -> Result<()> {
        self.dashboard.close()?;
        self.primary.close()?;
        self.secondary.close()?;
        self.rtde.close()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct UrPort {
    pub reader: BufReader<TcpStream>,
    pub writer: BufWriter<TcpStream>,
    socket: TcpStream,
}

impl UrPort {
    /// Create a new TCP connection to the selected port
    pub fn new(host: IpAddr, timeout: Option<Duration>, port: u16) -> Result<Self> {
        let stream = TcpStream::connect(SocketAddr::new(host, port))?;
        stream.set_read_timeout(timeout)?;
        stream.set_write_timeout(timeout)?;
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream.try_clone()?),
            socket: stream,
        })
    }
    /// Simple TCP Read of the port
    pub fn read(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        Ok(buf)
    }
    /// Simple TCP Write of command to the port, all writes also read response
    pub fn write(&mut self, command: &str) -> Result<String> {
        let payload = format!("{command}\n");
        self.writer.write_all(payload.as_bytes())?;
        self.writer.flush()?;
        self.read()
    }
    /// Close this socket
    pub fn close(self) -> Result<()> {
        match self.socket.shutdown(std::net::Shutdown::Both) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::net::Ipv4Addr;

    const TEST_PROGRAM: &str = "rtde_control_loop.urp";
    const TEST_PROGRAM_2: &str = "rtde_control_loop_copy.urp";
    const TIMEOUT: Duration = Duration::from_secs(10);

    fn init_ur() -> Result<UniversalRobot> {
        match UniversalRobot::connect(Ipv4Addr::LOCALHOST.into(), Duration::from_secs(10)) {
            Err(error) => panic!("{}", error),
            Ok(ur) => {
                assert!(!ur.dashboard.latest_message().is_empty());
                Ok(ur)
            }
        }
    }

    #[test]
    /// Test connecting and disconnecting from Universal Robot
    fn test_connecting() {
        let ur = init_ur().unwrap();
        print!(">>> latest message: {}", ur.dashboard.latest_message());
        assert!(ur.close().is_ok());
    }
    #[test]
    fn test_get_meta_data() {
        let mut ur = init_ur().unwrap();
        assert!(ur.get_meta_data().is_ok());
        assert!(ur.close().is_ok());
    }
    #[test]
    fn test_get_state_data() {
        let mut ur = init_ur().unwrap();
        assert!(ur.get_state().is_ok());
        assert!(ur.close().is_ok());
    }
    #[test]
    fn test_awaited_load() {
        let mut ur = init_ur().unwrap();
        println!("{}", ur.dashboard.get_loaded_program().unwrap());
        println!("{:?}", ur.dashboard.get_program_state().unwrap());
        if let Err(e) = ur.load(TEST_PROGRAM_2, TIMEOUT) {
            panic!("load 1 {e:?}");
        };
        if let Err(e) = ur.load(TEST_PROGRAM, TIMEOUT) {
            panic!("load 1 {e:?}");
        };
        assert!(ur.close().is_ok());
    }
    #[test]
    fn test_awaited_power_on() {
        let mut ur = init_ur().unwrap();
        assert!(ur.dashboard.power(false).is_ok());
        println!("{:?}", ur.dashboard.get_mode().unwrap());
        if let Err(e) = ur.power_on(TIMEOUT) {
            panic!("{}", e);
        };
        println!("Power On");
        assert!(ur.close().is_ok());
    }
}
