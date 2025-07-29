use super::types::{OpMode, ProgramState, RobotMode};
use crate::prelude::*;

/// Queries to ask the robot arm for state information
impl Dashboard {
    /// Robot Mode enquiry
    ///
    /// modes:
    ///
    /// no controller, disconnected, confirm safety,
    /// booting, power off, power on, idle, backdrive, running
    ///
    /// - supported from 5.0.0
    pub fn get_mode(&mut self) -> Result<RobotMode> {
        let response = self.send("robotmode", "robotmode")?;
        let mode: Vec<&str> = response.split_whitespace().collect();
        // expected response: "Robotmode: <mode>"
        if let Some(status) = mode.get(1) {
            match *status {
                "no_controller" => Ok(RobotMode::NoController),
                "disconnected" => Ok(RobotMode::Disconnected),
                "confirm_safety" => Ok(RobotMode::ConfirmSafety),
                "booting" => Ok(RobotMode::Booting),
                "power_off" => Ok(RobotMode::PowerOff),
                "power_on" => Ok(RobotMode::PowerOn),
                "idle" => Ok(RobotMode::Idle),
                "backdrive" => Ok(RobotMode::Backdrive),
                "running" => Ok(RobotMode::Running),
                val => Err(Error::UnexpectedResponse(format!(
                    "Unknown Robot Mode: {val}"
                ))),
            }
        } else {
            Err(Error::UnexpectedResponse(response))
        }
    }
    /// Execution state enquiry
    /// - supported from 5.0.0
    pub fn is_running(&mut self) -> Result<bool> {
        let response = self.send("running", "program running")?;
        let state: Vec<&str> = response.split_whitespace().collect();
        // expected response: "Program running: <bool>"
        match *state.last().unwrap_or(&"") {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /// Enquire about the save state of the active program and path to loaded program file
    /// - supported from 5.0.0
    pub fn is_saved(&mut self) -> Result<(bool, Option<String>)> {
        let response = self.send("isProgramSaved", "")?;
        // anticipated response "program running: false"
        if response.contains("program running: false") {
            return Ok((false, None));
        }
        let mut state = response.split_whitespace();
        let Some(status) = state.next() else {
            return Err(Error::UnexpectedResponse(response));
        };
        let Some(program_name) = state.next() else {
            return Err(Error::UnexpectedResponse(response));
        };
        // expected response: "true <program.name>" or "false <program.name>"
        match status {
            "true" => Ok((true, Some(program_name.to_owned()))),
            "false" => Ok((false, Some(program_name.to_owned()))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /// Returns the remote control status of the robot.
    ///
    /// If the robot is in local mode or disabled it returns false.
    /// - supported from 5.6.0
    pub fn is_remote_mode(&mut self) -> Result<bool> {
        let response = self.send("is in remote control", "")?;
        // expected response: "true", or "false"
        match response.as_str().trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /// Returns the state of the loaded program
    ///
    /// Stopped, Playing, Paused
    /// - supported from 5.0.0
    pub fn get_program_state(&mut self) -> Result<ProgramState> {
        let response = self.send("programState", "")?;
        // expected response: i.e "playing 'program.urp'"
        let mut state = response.split_whitespace();
        let Some(status) = state.next() else {
            return Err(Error::UnexpectedResponse(response));
        };
        let Some(program_name) = state.next() else {
            return Err(Error::UnexpectedResponse(response));
        };
        match status {
            "playing" => Ok(ProgramState::Playing(program_name.to_owned())),
            "paused" => Ok(ProgramState::Paused(program_name.to_owned())),
            "stopped" => {
                if program_name == "<unnamed>" {
                    Ok(ProgramState::Stopped(None))
                } else {
                    Ok(ProgramState::Stopped(Some(program_name.to_owned())))
                }
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /// Which program is loaded?
    /// - supported from 5.0.0
    pub fn get_loaded_program(&mut self) -> Result<String> {
        let response = self.send("get loaded program", "loaded program")?;
        // expected response: "Loaded program: <path to loaded program>
        let prog: Vec<&str> = response.split_whitespace().collect();
        let Some(path) = prog.get(2) else {
            return Err(Error::UnexpectedResponse(response));
        };
        Ok(path.trim_start_matches("/ursim/programs/").to_string())
    }
    /// Version information for the UR Software installed on the Robot
    /// - supported from 5.0.0
    pub fn get_version(&mut self) -> Result<String> {
        self.send("PolyscopeVersion", "URSoftware")
    }
    /// Serial number of Robot
    /// - supported from 5.6.0
    pub fn get_serial(&mut self) -> Result<String> {
        self.send("get serial number", "")
    }
    /// Robot model
    /// - supported from 5.6.0
    pub fn get_model(&mut self) -> Result<String> {
        self.send("get robot model", "")
    }
    /// Get the robot's operational mode
    ///
    /// Some(manual) or Some(automatic) if the password has been set for Mode in Settings.
    ///
    /// None if password not set.
    /// - supported from 5.6.0
    pub fn get_op_mode(&mut self) -> Result<Option<OpMode>> {
        let response = self.send("get operational mode", "")?;
        match response.as_str().trim() {
            "manual" => Ok(Some(OpMode::Manual)),
            "automatic" => Ok(Some(OpMode::Automatic)),
            "none" => Ok(None),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
