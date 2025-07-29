use super::types::OpMode;
use crate::prelude::*;

/// Commands used to set a state on the robot arm
impl Dashboard {
    /// Load a known program to the Robot
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn load_program(&mut self, program: &str) -> Result<String> {
        let payload = if program.ends_with(".urp") {
            format!("Load {program}")
        } else {
            format!("Load {program}.urp")
        };
        self.send(&payload, "loading program")
    }
    /// Load a known installation to the Robot
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn load_installation(&mut self, installation: Option<&str>) -> Result<String> {
        let payload;
        match installation {
            Some(installation) => {
                if installation.ends_with(".installation") {
                    payload = format!("load installation {installation}");
                } else {
                    payload = format!("load installation {installation}.installation");
                }
            }
            None => payload = "load installation default.installation".to_owned(),
        }
        self.send(&payload, "loading installation")
    }
    /// Play the loaded program to the Robot
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn play(&mut self) -> Result<String> {
        for _ in 0..5 {
            if let Ok(resp) = self.send("play", "starting program") {
                return Ok(resp);
            } else {
                sleep(Duration::from_millis(100));
            }
        }
        Err(Error::Static("failed to execute play command"))
    }
    /// Stop the loaded program to the Robot
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn stop(&mut self) -> Result<String> {
        self.send("stop", "stopped")
    }
    /// Pause the loaded program to the Robot
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn pause(&mut self) -> Result<String> {
        self.send("pause", "pausing program")
    }

    /// Shuts down and turns off robot and controller
    /// - supported from 5.0.0
    pub fn shutdown(&mut self) -> Result<String> {
        self.send("shutdown", "shutting down")
    }
    /// Open a popup on the robot tablet with the message
    /// - supported from 5.0.0
    pub fn popup_open(&mut self, message: &str) -> Result<String> {
        let payload = format!("popup {message}");
        self.send(&payload, "showing popup")
    }
    /// Closes an open popup
    /// - supported from 5.0.0
    pub fn popup_close(&mut self) -> Result<String> {
        self.send("close popup", "closing popup")
    }
    /// Adds message to log history
    /// - supported from 5.0.0
    pub fn log(&mut self, message: &str) -> Result<String> {
        let payload = format!("addToLog {message}");
        self.send(&payload, "added log message")
    }
    /// Set the operational mode of the robot
    /// - Some(manual) = Loading and editing programs is allowed
    /// - Some(automatic) = Loading and editing programs and installations is not allowed, only playing programs
    /// - None = operational mode is no longer controlled by Dashboard Server
    ///
    /// If this function is called the operational mode cannot be changed from PolyScope, and the user password is disabled.
    /// - supported from 5.0.0
    pub fn set_op_mode(&mut self, mode: Option<OpMode>) -> Result<String> {
        match mode {
            Some(mode) => {
                let mode_str = match mode {
                    OpMode::Manual => "manual",
                    OpMode::Automatic => "automatic",
                };
                let payload = format!("set operational mode {mode_str}");
                let response_pattern = format!("operational mode '{mode_str}' is set");
                self.send(&payload, &response_pattern)
            }
            None => self.send(
                "clear operational mode",
                "no longer controlling the operational mode",
            ),
        }
    }
    /// Set Power state to robot arm
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn power(&mut self, on: bool) -> Result<String> {
        match on {
            true => self.send("power on", "powering on"),
            false => self.send("power off", "powering off"),
        }
    }
    /// Release the brakes
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn brake_release(&mut self) -> Result<String> {
        self.send("brake release", "brake releasing")
    }
}
