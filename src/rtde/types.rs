use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_repr::*;

use super::data::DataType;
use crate::prelude::*;

/// RTDE Package Types
#[repr(u8)]
#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone, Copy)]
pub enum PackageType {
    ProtocolVersion = 86,
    URControlVersion = 118,
    Message = 77,
    Data = 85,
    /// Output from Robot
    SetupOutputs = 79,
    /// Input to Robot
    SetupInputs = 73,
    Start = 83,
    Pause = 80,
}

impl TryFrom<u8> for PackageType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            86 => Ok(PackageType::ProtocolVersion),
            118 => Ok(PackageType::URControlVersion),
            77 => Ok(PackageType::Message),
            85 => Ok(PackageType::Data),
            79 => Ok(PackageType::SetupOutputs),
            73 => Ok(PackageType::SetupInputs),
            83 => Ok(PackageType::Start),
            80 => Ok(PackageType::Pause),
            num => Err(Error::UnexpectedResponse(format!(
                "Unknown Package Type: {}",
                num
            ))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    bugfix: u32,
    build: u32,
}

#[repr(u16)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
pub enum Protocol {
    V1 = 1,
    V2 = 2,
}

pub struct Recipe {
    id: u8,
    var_types: Vec<DataType>,
}

impl Recipe {
    pub fn new(id: u8, var_types: Vec<DataType>) -> Self {
        Self { id, var_types }
    }
    pub fn get_types(&self) -> Vec<DataType> {
        self.var_types.clone()
    }
    pub fn id(&self) -> u8 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub package_size: u16,
    pub package_type: PackageType,
}

impl Header {
    /// Create a new header for commands with no data payload
    pub fn new(package_type: PackageType, set_size: Option<u16>) -> Self {
        Self {
            package_size: set_size.unwrap_or(3),
            package_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<T: Serialize> {
    header: Header,
    pub payload: T,
}

impl<T> Payload<T>
where
    T: Sized + Serialize + DeserializeOwned,
{
    pub fn new(package_type: PackageType, package: T, set_size: Option<u16>) -> Result<Self> {
        let mut payload = Payload {
            header: Header {
                package_size: 0,
                package_type,
            },
            payload: package,
        };
        if let Some(size) = set_size {
            payload.header.package_size = size;
            Ok(payload)
        } else {
            Ok(payload)
        }
        // } else if let Ok(size) = bincode::serialized_size(&payload) {
        //     // TODO: Currently Not imleted in Bincode 2 ^
        //     payload.header.package_size = size as u16;
        //     Ok(payload)
        // } else {
        //     Err(Error::Static("issue setting payload size"))
        // }
    }
    /// What type of Payload package do we have?
    pub fn get_type(&self) -> PackageType {
        self.header.package_type
    }
    /// Check if the package type is Data, in which case we need to extract its recipe ID
    pub fn is_data(&self) -> bool {
        self.header.package_type == PackageType::Data
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]
pub enum Level {
    Exception,
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    message: String,
    source: String,
    level: Level,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            message: Default::default(),
            source: Default::default(),
            level: Level::Info,
        }
    }
}

impl Message {
    pub fn new(message: &str, source: &str, level: Level) -> Message {
        Message {
            message: message.to_owned(),
            source: source.to_owned(),
            level,
        }
    }
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        match bincode::serde::encode_to_vec(self, config) {
            Err(error) => Err(Error::Serialization(error.to_string())),
            Ok(bytes) => Ok(bytes),
        }
    }
}
