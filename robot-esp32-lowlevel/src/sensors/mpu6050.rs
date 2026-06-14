use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::sys::EspError;

// https://cdn.sparkfun.com/datasheets/Sensors/Accelerometers/RM-MPU-6000A.pdf

// Addresses
const MPU_ADDR: u8 = 0x68;

// Registers
const POWER_MAN_1_REG: u8 = 0x6B;
const WHO_AM_I: u8 = 0x75;
const ACCEL_REG: u8 = 0x3B;
const GYRO_REG: u8 = 0x43;

// Other
const DEFAULT_TIMEOUT: u32 = 500;

/// Wakes up Mpu6050 by taking in a already made I2c driver and turning it on
pub fn wake(ic2_driver: &mut I2cDriver) -> Result<(), EspError> {
    let mut buffer = [0u8; 1];
    ic2_driver.write_read(MPU_ADDR, &[POWER_MAN_1_REG], &mut buffer, DEFAULT_TIMEOUT)?;
    // Wake is bit 6, 0 is on, 1 is zzz
    let modifed = buffer[0] & 0b10111111;
    ic2_driver.write(MPU_ADDR, &[POWER_MAN_1_REG, modifed], DEFAULT_TIMEOUT)
}

/// Returns a u8 which is the drivers name
pub fn who_am_i(ic2_driver: &mut I2cDriver) -> Result<u8, EspError> {
    let mut buffer = [0u8; 1];
    ic2_driver.write_read(MPU_ADDR, &[WHO_AM_I], &mut buffer, DEFAULT_TIMEOUT)?;
   
   Ok(buffer[0])
}

/// Returns accel data which is a 3 sized array with accel data in x,y,z format each i16
pub fn get_accel_data(ic2_driver: &mut I2cDriver) -> Result<[i16; 3], EspError> {
    let mut return_value = [0i16; 3];
    let mut buffer = [0u8; 6];
    ic2_driver.write_read(MPU_ADDR, &[ACCEL_REG], &mut buffer, DEFAULT_TIMEOUT)?;
    // x
    let raw_x = (buffer[0] as u16) << 8 | (buffer[1] as u16);
    return_value[0] = raw_x as i16;
    // y
    let raw_y = (buffer[2] as u16) << 8 | (buffer[3] as u16);
    return_value[1] = raw_y as i16;
    // z
    let raw_z = (buffer[4] as u16) << 8 | (buffer[5] as u16);
    return_value[2] = raw_z as i16;

    Ok(return_value)
}

/// Returns gyro data which is a 3 sized array with gyro data in x,y,z format each i16
pub fn get_gyro_data(ic2_driver: &mut I2cDriver) -> Result<[i16; 3], EspError> {
    let mut return_value = [0i16; 3];
    let mut buffer = [0u8; 6];
    ic2_driver.write_read(MPU_ADDR, &[GYRO_REG], &mut buffer, DEFAULT_TIMEOUT)?;
    // x
    let raw_x = (buffer[0] as u16) << 8 | (buffer[1] as u16);
    return_value[0] = raw_x as i16;
    // y
    let raw_y = (buffer[2] as u16) << 8 | (buffer[3] as u16);
    return_value[1] = raw_y as i16;
    // z
    let raw_z = (buffer[4] as u16) << 8 | (buffer[5] as u16);
    return_value[2] = raw_z as i16;

    Ok(return_value)
}