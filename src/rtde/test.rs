use std::net::Ipv4Addr;

use crate::prelude::*;
use crate::rtde::data::{DataType, DEFAULT_OUTPUTS};
use crate::rtde::types::{Level, Protocol};
use crate::Rtde;

fn init_rtde() -> Result<Rtde> {
    Rtde::new(
        std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Some(Duration::from_secs(10)),
    )
}

#[test]
fn test_connect_rtde() {
    let rtde = init_rtde().unwrap();
    assert!(rtde.close().is_ok());
}
#[test]
fn test_set_protocol() {
    let mut rtde = init_rtde().unwrap();
    rtde.set_protocol_version(Protocol::V2).unwrap();
    assert!(rtde.close().is_ok());
}
#[test]
fn test_send_message() {
    let mut rtde = init_rtde().unwrap();
    assert!(rtde
        .send_message("Hello World", "Rust", Level::Info)
        .is_ok());
    assert!(rtde.close().is_ok());
}
#[test]
fn test_setup_inputs() {
    let mut rtde = init_rtde().unwrap();
    let recipe = vec!["input_int_register_0"];
    let resp = rtde.setup_input(&recipe).unwrap();
    println!("{},{:?}", resp.id(), resp.get_types());
    let recipe = vec!["input_double_register_1"];
    let resp = rtde.setup_input(&recipe).unwrap();
    println!("{},{:?}", resp.id(), resp.get_types());
    assert!(rtde.close().is_ok());
}
#[test]
fn test_start_stop() {
    let mut rtde = init_rtde().unwrap();
    let resp = rtde.setup_output(&DEFAULT_OUTPUTS, 500.0).unwrap();
    assert_eq!(
        resp.get_types(),
        vec![
            DataType::U64,
            DataType::F64,
            DataType::Vec6,
            DataType::Vec6,
            DataType::Vec6,
            DataType::I32,
            DataType::I32
        ]
    );
    println!("{:?}", resp.get_types());
    assert!(rtde.start().is_ok());
    sleep(Duration::from_secs(2));
    // assert!(rtde.pause().is_ok());
    rtde.pause().unwrap();
    assert!(rtde.close().is_ok());
}
