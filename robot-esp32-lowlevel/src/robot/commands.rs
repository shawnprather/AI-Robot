use esp_idf_svc::{hal::ledc::LedcDriver, sys::EspError};

use crate::{robot::robot::Robot, servos::mg90s};

pub fn parse_command(command: &str, robot: &Robot) -> Option<Box<dyn Fn()>> {
    let args: Vec<&str> = command.trim().split_whitespace().collect();
    match args[0] {
        "SERVO" => {
            
        },
        _ => {
            None
        }
    }
}