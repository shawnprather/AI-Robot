use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::ledc::LedcDriver;

use std::fmt;

use crate::I2cDriver;
use crate::sensors::mpu6050::PositionReading;
use crate::sensors::sht31::Sht31;
use crate::sensors::mpu6050::Mpu6050;
use crate::sensors::sht31::WeatherReading;

pub struct Robot {
    pub i2c: I2cDriver<'static>,
    pub uart: Arc<Mutex<UartDriver<'static>>>,
    pub mpu: Option<Mpu6050>,
    pub sht: Option<Sht31>,
    pub servos: HashMap<String, LedcDriver<'static>>
}

pub struct RobotReadings {
    pub position: Option<PositionReading>,
    pub weather: Option<WeatherReading>,
}

impl fmt::Display for RobotReadings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(position) = &self.position {
            write!(f, "{} \n", position)?;
        }
        if let Some(weather) = &self.weather {
            write!(f, "{} \n", weather)?;
        }
        return Ok(())        
    }
}

