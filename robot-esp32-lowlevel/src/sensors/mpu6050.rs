use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::sys::EspError;
use esp_idf_svc::hal::delay::FreeRtos;

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
const DEFAULT_N_CALIBRATION: usize = 40;

// Info struct
pub struct Reading {
    pub accel: [f32; 3],
    pub gyro: [f32; 3],
    pub accel_magnitude: f32
}

// MPU struct
pub struct Mpu6050 {
    bias_x: f32,
    bias_y: f32,
    bias_z: f32,
}

impl Mpu6050 {
    pub fn new(i2c: &mut I2cDriver) -> Result<Self, EspError> {
        let mut bias_x_list = [0i16; DEFAULT_N_CALIBRATION];
        let mut bias_y_list = [0i16; DEFAULT_N_CALIBRATION];
        let mut bias_z_list = [0i16; DEFAULT_N_CALIBRATION];

        wake(i2c)?;

        // Calculate the bias
        for i in 0..DEFAULT_N_CALIBRATION {
            let raw_gyro = get_raw_gyro(i2c)?;
            bias_x_list[i] = raw_gyro[0];
            bias_y_list[i] = raw_gyro[1];
            bias_z_list[i] = raw_gyro[2];
            FreeRtos::delay_ms(10);
        }

        let bias_x = bias_x_list.iter().sum::<i16>() as f32 / bias_x_list.len() as f32;
        let bias_y = bias_y_list.iter().sum::<i16>() as f32 / bias_y_list.len() as f32;
        let bias_z = bias_z_list.iter().sum::<i16>() as f32 / bias_z_list.len() as f32;

        Ok(Mpu6050{ bias_x, bias_y, bias_z })
    }

    /// Returns gyro data which is a 3 sized array with gyro data in x,y,z format each f32
    fn get_gyro_data(&self, ic2_driver: &mut I2cDriver) -> Result<[f32; 3], EspError> {
        let raw_gyro = get_raw_gyro(ic2_driver)?;
        let gyro_x = (raw_gyro[0] as f32 - self.bias_x) / 131.0;
        let gyro_y = (raw_gyro[1] as f32 - self.bias_y) / 131.0;
        let gyro_z = (raw_gyro[2] as f32 - self.bias_z) / 131.0;

        Ok([gyro_x, gyro_y, gyro_z])
    }

    fn get_accel_data(&self, ic2_driver: &mut I2cDriver) -> Result<[f32; 3], EspError> {
        let raw_accel = get_raw_accel(ic2_driver)?;
        let accel_x = (raw_accel[0] as f32) / 16384.0;
        let accel_y = (raw_accel[1] as f32) / 16384.0;
        let accel_z = (raw_accel[2] as f32) / 16384.0;

        Ok([accel_x, accel_y, accel_z])
    }

    pub fn get_mpu_state(&self, i2c: &mut I2cDriver) -> Result<Reading, EspError> {
        let gyro = self.get_gyro_data(i2c)?;
        let accel = self.get_accel_data(i2c)?;
        let accel_magnitude = ((accel[0] as f32).powi(2) + (accel[1] as f32).powi(2) + (accel[2] as f32).powi(2)).sqrt();
        Ok(Reading { accel, gyro, accel_magnitude })
    }
}


/// Wakes up Mpu6050 by taking in a already made I2c driver and turning it on
fn wake(ic2_driver: &mut I2cDriver) -> Result<(), EspError> {
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

/// Returns raw accel data which is a 3 sized array with accel data in x,y,z format each i16
fn get_raw_accel(ic2_driver: &mut I2cDriver) -> Result<[i16; 3], EspError> {
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

/// Returns raw gyro data which is a 3 sized array with gyro data in x,y,z format each i16
fn get_raw_gyro(ic2_driver: &mut I2cDriver) -> Result<[i16; 3], EspError> {
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