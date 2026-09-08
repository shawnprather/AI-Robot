use crate::I2cDriver;
use crate::sensors::sht31::Sht31;
use crate::sensors::mpu6050::Mpu6050;

pub struct Robot {
    pub i2c: I2cDriver<'static>,
    pub mpu: Option<Mpu6050>,
    pub sht: Option<Sht31>
}