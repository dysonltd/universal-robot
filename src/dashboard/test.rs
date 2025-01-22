use crate::dashboard::types::OpMode;
use crate::prelude::*;
use std::net::Ipv4Addr;

const TEST_PROGRAM: &str = "rtde_control_loop.urp";

fn init_dash() -> Result<Dashboard> {
    let dashboard = Dashboard::new(
        std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Some(Duration::from_secs(10)),
    )?;
    assert!(!dashboard.latest_message().is_empty());
    Ok(dashboard)
}

#[test]
fn test_get_mode() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.get_mode().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_is_running() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.is_running().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_is_saved() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.is_saved().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_is_remote_mode() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.is_remote_mode().is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_get_loaded_program() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.get_loaded_program().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_get_version() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.get_version().is_ok());
    assert!(dashboard.get_serial().is_ok());
    assert!(dashboard.get_model().is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_get_op_mode() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.get_op_mode().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_power_cycle() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.power(true).is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_log() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.log("testing").is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_set_op_mode() {
    let mut dashboard = init_dash().unwrap();
    let modes = [Some(OpMode::Manual), Some(OpMode::Automatic), None];
    for mode in modes {
        assert!(dashboard.set_op_mode(mode).is_ok());
        assert_eq!(dashboard.get_op_mode().unwrap(), mode);
        sleep(Duration::from_secs(1));
    }
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_load_program() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.load_program(TEST_PROGRAM).is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_release_brakes() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.brake_release().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_popup() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.popup_open("hello from Rust").is_ok());
    sleep(Duration::from_secs(2));
    assert!(dashboard.popup_close().is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_load_installation() {
    let mut dashboard = init_dash().unwrap();
    // assert!(dashboard.load_installation(None).is_ok());
    let resp = dashboard.load_installation(None).unwrap();
    println!("{}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_play_program() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.get_loaded_program().is_ok());
    assert!(dashboard.power(true).is_ok());
    sleep(Duration::from_secs(5));
    assert!(dashboard.brake_release().is_ok());
    sleep(Duration::from_secs(5));
    assert!(dashboard.play().is_ok(), "play");
    sleep(Duration::from_secs(1));
    assert!(dashboard.pause().is_ok(), "pause");
    sleep(Duration::from_secs(1));
    assert!(dashboard.play().is_ok(), "play");
    sleep(Duration::from_secs(1));
    assert!(dashboard.stop().is_ok(), "stop");
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_get_program_state() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.load_program(TEST_PROGRAM).is_ok());
    let resp = dashboard.get_program_state().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_get_safety_status() {
    let mut dashboard = init_dash().unwrap();
    let resp = dashboard.safety_status().unwrap();
    println!("{:?}", resp);
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_safety_popup_close() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.safety_popup_close().is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_unlock_safety() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.safety_unlock_protective_stop().is_ok());
    assert!(dashboard.close().is_ok());
}
#[test]
fn test_restart_safety() {
    let mut dashboard = init_dash().unwrap();
    assert!(dashboard.safety_restart().is_ok());
    assert!(dashboard.close().is_ok());
}
