//! Expects to be talking to a locally hosted Simulator of a Universal Robot.
//!
//! This can be built by running:
//! `docker compose -f "./examples/simulator/docker-compose.yaml" up --build`
//!
//! You can then see the virtual UI of the robot at:
//! http://172.19.0.2:6080/vnc.html?host=172.19.0.2&port=6080
//!
//! This will copy in examples/simulator/programs/*.urp as an example programs to load and talk to.

use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;
use std::{error::Error, net::Ipv4Addr};

use serde::Deserialize;
use universal_robot::data::Vec6;
use universal_robot::prelude::*;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct Output {
    digital_bits: u64,
    timestamp: f64,
    tcp_pose: Vec6,
    tcp_speed: Vec6,
    joint_poses: Vec6,
    safety_mode: i32,
    robot_mode: i32,
    is_ready: i32,
}
const OUTPUTS: &[&str] = &[
    "actual_digital_output_bits",
    "timestamp",
    "actual_TCP_pose",
    "actual_TCP_speed",
    "actual_q",
    "safety_mode",
    "robot_mode",
    "output_int_register_0", // is_ready
];

const ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const TIMEOUT: Duration = Duration::from_secs(10);
const DATA_RATE_HZ: f64 = 50.0;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    log::info!("Running Demo");

    let mut ur = UniversalRobot::connect(ADDRESS, Duration::from_secs(5))?;
    log::info!("Controller version: {:?}", ur.dashboard.get_version()?);
    println!("Powering on...");
    ur.power_on(TIMEOUT)?;
    println!("Loading program onto Robot");
    ur.load("rtde_control_loop.urp", TIMEOUT)?;
    ur.info("Connected to Rust", "demo.rs")?;
    ur.dashboard.popup_open("Hello from Rust")?;
    sleep(Duration::from_secs(2)); // slow down program so state changes can be seen in the simulator
    ur.dashboard.popup_close()?;

    // Set up RTDE Recipies for communicating real-time data with the Robot
    let output_recipe = ur.rtde.setup_output(OUTPUTS, DATA_RATE_HZ)?;
    println!("output types: {:?}", output_recipe.get_types());
    println!("-----");
    ur.rtde.start()?;
    let input_recipe_1 = ur.rtde.setup_input(&["input_int_register_0"])?; // start move flag
    println!("input types: {:?}", input_recipe_1.get_types());
    println!("-----");
    let input_recipe_2 = ur.rtde.setup_input(&[
        "input_double_register_0",
        "input_double_register_1",
        "input_double_register_2",
        "input_double_register_3",
        "input_double_register_4",
        "input_double_register_5",
    ])?;
    println!("input types: {:?}", input_recipe_2.get_types());
    println!("-----");

    let pose_1 = Vec6::new(-0.12, -0.43, 0.14, 0.0, 3.11, 0.04);
    let pose_2 = Vec6::new(-0.12, -0.51, 0.21, 0.0, 3.11, 0.04);

    // Start the program
    ur.rtde.write(0i32, input_recipe_1.id())?; // set initial value of recipe
    ur.rtde.write(Vec6::default(), input_recipe_2.id())?; // set initial value of recipe
    ur.dashboard.stop()?;
    ur.dashboard.brake_release()?;
    ur.dashboard.play()?;

    sleep(Duration::from_secs(2)); // slow down program so state changes can be seen in the simulator

    let mut move_completed = true;
    let mut index = 0;
    while index < 10 {
        let Ok(state) = ur.rtde.read()?.parse::<Output>() else {
            break;
        };
        if move_completed && state.is_ready == 1 {
            move_completed = false;
            let pose = if index % 2 == 0 { pose_1 } else { pose_2 };
            index += 1;
            ur.rtde.write(pose, input_recipe_2.id())?;
            ur.rtde.write(1i32, input_recipe_1.id())?; // set initial value of recipe
        } else if !move_completed && state.is_ready == 0 {
            println!("move to confirmed pose = {:?}", state.tcp_pose);
            move_completed = true;
            ur.rtde.write(0i32, input_recipe_1.id())?;
        }
        println!("{:.3?}", state);
    }

    sleep(Duration::from_secs(2)); // slow down program so state changes can be seen in the simulator
    ur.rtde.pause()?;
    ur.dashboard.stop()?;
    sleep(Duration::from_secs(2)); // slow down program so state changes can be seen in the simulator
    ur.info("Disconnected from Rust", "demo.rs")?;
    ur.close()?;
    Ok(())
}
