use super::types::SafetyStatus;
use crate::prelude::*;

/// Commands and Queries used for safety related features
impl Dashboard {
    /// Safety Status Inquiry
    /// - supported from 5.4.0
    pub fn safety_status(&mut self) -> Result<SafetyStatus> {
        let response = self.send("safetystatus", "safetystatus")?;
        let status: Vec<&str> = response.split_whitespace().collect();
        // expected response: "safetystatus: <status>"
        match status[1] {
            "normal" => Ok(SafetyStatus::Normal),
            "reduced" => Ok(SafetyStatus::Reduced),
            "protective_stop" => Ok(SafetyStatus::ProtectiveStop),
            "recovery" => Ok(SafetyStatus::Recovery),
            "safeguard_stop" => Ok(SafetyStatus::SafeguardStop),
            "system_emergency_stop" => Ok(SafetyStatus::SystemEmergencyStop),
            "robot_emergency_stop" => Ok(SafetyStatus::RobotEmergencyStop),
            "violation" => Ok(SafetyStatus::Violation),
            "fault" => Ok(SafetyStatus::Fault),
            "automatic_mode_safeguard_stop" => Ok(SafetyStatus::AutomaticModeSafeguardStop),
            "system_three_position_enabling_stop" => {
                Ok(SafetyStatus::SystemThreePositionEnablingStop)
            }
            val => Err(Error::UnexpectedResponse(format!(
                "Unknown Safety Status: {val}"
            ))),
        }
    }
    /// Closes an open Safety Popup
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn safety_popup_close(&mut self) -> Result<String> {
        self.send("close safety popup", "closing safety popup")
    }
    /// Closes the current popup and unlocks protective stop.
    ///
    /// Fails if less than 5 seconds has passed since the protective stop occured.
    /// - Remote control only
    /// - supported from 5.0.0
    pub fn safety_unlock_protective_stop(&mut self) -> Result<String> {
        self.send("unlock protective stop", "protective stop releasing")
    }
    /// Used when robot gets a safety fault or violation to restart the safety.
    /// After safety has been rebooted the robot will be in Power Off.
    ///
    /// <b>You should always ensure it is okay to restart the system.
    /// It is highly recommended to check the error log before using this command.</b>
    /// - Remote control only
    /// - supported from 5.1.0
    pub fn safety_restart(&mut self) -> Result<String> {
        self.log("restarted safety remotely")?;
        self.send("restart safety", "restarting safety")
    }
}
