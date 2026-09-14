use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::ledc::LedcChannel;
use esp_idf_svc::hal::ledc::LedcDriver;
use esp_idf_svc::hal::ledc::LedcTimer;
use esp_idf_svc::hal::ledc::LedcTimerDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::units::Hertz;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

use crate::robot::robot::Robot;
use crate::robot::robot::RobotReadings;
use crate::robot::uart;
use crate::sensors::sht31::Sht31;
use crate::sensors::mpu6050::Mpu6050;

mod sensors;
mod servos;
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

    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio8,
        peripherals.pins.gpio9,
        &I2cConfig::new().baudrate(Hertz(6000))
    ).unwrap();
    
    let mpu= Mpu6050::new(&mut i2c)
        .map_err(|e| log::error!("MPU init failed: {:?}", e))
        .ok();

    let sht = Sht31::new()
        .map_err(|e| log::error!("SHT init failed {:?}", e))
        .ok();

    let uart = Arc::new(Mutex::new(
        UartDriver::new(
            peripherals.uart0,
            peripherals.pins.gpio15,
            peripherals.pins.gpio16,
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &UartConfig::new()
        ).unwrap()
    ));

    let mut servos: HashMap<&str, LedcDriver> = HashMap::new();
    servos.insert("", )

    Robot{ i2c, uart, mpu, sht }
}

fn main() {
    let robot = startup();

    let Robot { mut i2c, mpu, sht, uart: uart_sensor } = robot;
    let uart_command = Arc::clone(&uart_sensor);

    // Sensor Info
    thread::spawn(move || {
        let mut robot_readings = RobotReadings{ position: None, weather: None };
        loop {
            FreeRtos::delay_ms(1000);

            if let Some(mpu) = &mpu {
                match mpu.get_mpu_state(&mut i2c) {
                    Ok(position) => { robot_readings.position = Some(position) },
                    Err(e) => { log::error!("MPU read failed: {:?}", e)}
                }
            }

            if let Some(sht) = &sht {
                match sht.read_data(&mut i2c) {
                    Ok(weather) => { robot_readings.weather = Some(weather) },
                    Err(e) => { log::error!("SHT read failed: {:?}", e)}
                }
            }

            uart::write_to_uart(&mut uart_sensor.lock().unwrap(), &robot_readings.to_string());
        }
    });

    // Execute Commands
    thread::spawn(move || {
        let command = { uart::read_from_uart(&mut uart_command.lock().unwrap()) };
        if let Some(command) = command {

        }
        // Send Ok
        // Execute Command
    }); 
}
