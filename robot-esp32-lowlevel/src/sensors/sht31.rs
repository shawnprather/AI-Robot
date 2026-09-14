use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::sys::EspError;

use std::fmt;

// https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf

// Addresses
const SHT_ADDR: u8 = 0x44;

// Registers
const MEASURE_CMD_MSB: u8 = 0x2C;
const MEASURE_CMD_LSB: u8 = 0x06;

// Other
const DEFAULT_TIMEOUT: u32 = 500;

/// Raw Reading which holds both the raw temp and raw humidity
struct RawReading {
    pub raw_temperature: u16,
    pub raw_humidity: u16,
}

/// Reading which holds both the temp and humidity
pub struct WeatherReading {
    pub temperature: f32,
    pub humidity: f32,
}

impl fmt::Display for WeatherReading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SHT: temp {:.2} humidity {:.2}", self.temperature, self.humidity)
    }
}

pub struct Sht31 {}

impl Sht31 {
    pub fn new() -> Result<Self, EspError> {
        Ok(Sht31 {})
    }

    /// This function takes the I2C driver and does a clock streched high accuracy info pull and returns the Reading struct
    pub fn read_data(&self, i2c_driver: &mut I2cDriver) -> Result<WeatherReading, EspError> {
        let raw_data = read_raw_data(i2c_driver)?;
        let humidity = ((raw_data.raw_humidity as f32) / 65535.0) * 100.0;
        let temperature = (((raw_data.raw_temperature as f32) / 65535.0) * 315.0) - 49.0;
        
        Ok(WeatherReading { temperature, humidity })
    }
}

/// This function takes the I2C driver and does a clock streched high accuracy info pull and returns the Raw Reading struct
fn read_raw_data(i2c_driver: &mut I2cDriver) -> Result<RawReading, EspError> {
    let mut buffer = [0u8; 6];
    i2c_driver.write_read(SHT_ADDR, &[MEASURE_CMD_MSB, MEASURE_CMD_LSB], &mut buffer, DEFAULT_TIMEOUT)?;
    let raw_temperature = (buffer[0] as u16) << 8 | (buffer[1] as u16);
    let raw_humidity= (buffer[3] as u16) << 8 | (buffer[4] as u16);
    
    Ok(RawReading { raw_temperature, raw_humidity})
}