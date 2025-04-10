use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::types::Payload;
use crate::prelude::*;

#[repr(u16)]
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq)]
/// Variable types for decoding custom variables
pub enum DataType {
    Vec6,
    Vec3,
    IVec6,
    UVec6,
    F64,
    U64,
    U32,
    I32,
    Bool,
    U8,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec6 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

impl Vec6 {
    /// coords to mm and degrees
    pub fn convert(&self) -> Vec6 {
        Vec6 {
            x: self.x * 1000.0,
            y: self.y * 1000.0,
            z: self.z * 1000.0,
            rx: self.rx * 57.2958,
            ry: self.ry * 57.2958,
            rz: self.rz * 57.2958,
        }
    }
    pub fn new(x: f64, y: f64, z: f64, rx: f64, ry: f64, rz: f64) -> Self {
        Vec6 {
            x,
            y,
            z,
            rx,
            ry,
            rz,
        }
    }
}

pub const DEFAULT_OUTPUTS: [&str; 7] = [
    "actual_digital_output_bits",
    "timestamp",
    "actual_TCP_pose",
    "actual_TCP_speed",
    "actual_q",
    "safety_mode",
    "robot_mode",
];

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultOutputs {
    digital_bits: u64,
    timestamp: f64,
    tcp_pose: Vec6,
    tcp_speed: Vec6,
    joint_positions: Vec6,
    safety_mode: i32,
    robot_mode: i32,
}

impl DataType {
    /// convert from string to DataType Enum
    pub fn new(var_type: &str) -> DataType {
        match var_type.to_lowercase().as_str() {
            "vector6d" => DataType::Vec6,
            "vector3d" => DataType::Vec3,
            "vector6int32" => DataType::IVec6,
            "vector6uint32" => DataType::UVec6,
            "double" => DataType::F64,
            "uint64" => DataType::U64,
            "uint32" => DataType::U32,
            "int32" => DataType::I32,
            "bool" => DataType::Bool,
            "uint8" => DataType::U8,
            _ => DataType::NotFound,
        }
    }
}

impl Payload<Vec<u8>> {
    pub fn parse<T: DeserializeOwned>(&self) -> Result<T> {
        let data = if self.is_data() {
            &self.payload[1..] // remove recipe ID from the payload before decode
        } else {
            &self.payload
        };
        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        match bincode::serde::decode_from_slice(data, config) {
            Ok((data, _)) => Ok(data),
            Err(error) => Err(Error::Deserialization(error.to_string())),
        }
    }
}
