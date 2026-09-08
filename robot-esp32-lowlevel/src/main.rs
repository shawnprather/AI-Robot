use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::Pull;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::units::Hertz;

use crate::robot::robot::Robot;

mod sensors;
mod robot;

fn startup() -> Robot {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
 
    log::info!("Powered On");
    FreeRtos::delay_ms(1000);

    let peripherals = Peripherals::take().unwrap();

    // I2c Setup
    let config = I2cConfig::new().baudrate(Hertz(6000));
    let sda = peripherals.pins.gpio8;
    let scl = peripherals.pins.gpio9;
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    // let mpu = mpu6050::Mpu6050::new(&mut i2c_driver).unwrap_or_else(|e| {
    //     log::error!("MPU init failed: {:?}", e);
    //     panic!("MPU failed error");
    // });

    Robot{ i2c, mpu: None, sht: None }
}

fn main() {
    startup();
    
    loop {

        // match mpu.get_mpu_state(&mut i2c_driver) {
        //     Ok(reading) => log::info!("Gyro: [{:.0}] [{:.0}] [{:.0}], Accel: [{:.2}] [{:.2}] [{:.2}], Magnitude: [{:.2}]", reading.gyro[0], reading.gyro[1], reading.gyro[2], reading.accel[0], reading.accel[1], reading.accel[2], reading.accel_magnitude),
        //     Err(e) => log::error!("I2C error, get gyro: {:?}", e),
        // }

        // match sensors::sht31::read_data(&mut i2c_driver) {
        //     Ok(data) => log::info!("Data Temp: {}, Humid {}", data.temperature, data.humidity),
        //     Err(e) => log::error!("I2C error, read_data: {:?}", e),
        // }

        //FreeRtos::delay_ms(100);
    }
    
}
