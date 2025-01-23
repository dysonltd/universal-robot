use crate::prelude::UniversalRobot;
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(10);
const ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[test]
fn test_get_ur_version() {
    let mut ur = UniversalRobot::connect(ADDRESS, TIMEOUT).unwrap();
    let resp = ur.get_ur_version().unwrap();
    println!("{:?}", resp);
    ur.close().unwrap();
}

#[test]
fn test_send_info() {
    let mut ur = UniversalRobot::connect(ADDRESS, TIMEOUT).unwrap();
    ur.info("Hello World", "Rust").unwrap();
    ur.close().unwrap();
}

#[test]
fn test_send_error() {
    let mut ur = UniversalRobot::connect(ADDRESS, TIMEOUT).unwrap();
    ur.error("Hello World", "Rust").unwrap();
    ur.close().unwrap();
}

#[test]
fn test_send_warn() {
    let mut ur = UniversalRobot::connect(ADDRESS, TIMEOUT).unwrap();
    ur.warn("Hello World", "Rust").unwrap();
    ur.close().unwrap();
}

#[test]
fn test_send_exception() {
    let mut ur = UniversalRobot::connect(ADDRESS, TIMEOUT).unwrap();
    ur.exception("Hello World", "Rust").unwrap();
    ur.close().unwrap();
}
