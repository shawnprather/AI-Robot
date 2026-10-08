use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::ledc::LedcDriver;
use esp_idf_svc::hal::ledc::LedcTimerDriver;
use esp_idf_svc::hal::ledc::Resolution;
use esp_idf_svc::hal::ledc::config::TimerConfig;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::units::Hertz;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

use crate::robot::commands;
use crate::robot::commands::ENDLINE;
use crate::robot::commands::ERR;
use crate::robot::commands::OK;
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
        &I2cConfig::new().baudrate(Hertz(100000))
    ).unwrap();
    
    let mpu= Mpu6050::new(&mut i2c)
        .map_err(|e| log::error!("MPU init failed: {:?}", e))
        .ok();

    let sht = Sht31::new()
        .map_err(|e| log::error!("SHT init failed {:?}", e))
        .ok();

    let uart = Arc::new(Mutex::new(
        UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio15,
            peripherals.pins.gpio16,
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &UartConfig::new().baudrate(Hertz(115_200))
        ).unwrap()
    ));

    let mut servos: HashMap<&str, LedcDriver> = HashMap::new();
    let timer_config = TimerConfig::new().frequency(Hertz(50)).resolution(Resolution::Bits14);
    let ledc_timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config).unwrap();
    servos.insert("QU", LedcDriver::new(peripherals.ledc.channel0, &ledc_timer, peripherals.pins.gpio2).unwrap());
    servos.insert("QL", LedcDriver::new(peripherals.ledc.channel1, &ledc_timer, peripherals.pins.gpio42).unwrap());
    servos.insert("EU", LedcDriver::new(peripherals.ledc.channel2, &ledc_timer, peripherals.pins.gpio41).unwrap());
    servos.insert("EL", LedcDriver::new(peripherals.ledc.channel3, &ledc_timer, peripherals.pins.gpio40).unwrap());
    servos.insert("ZU", LedcDriver::new(peripherals.ledc.channel4, &ledc_timer, peripherals.pins.gpio39).unwrap());
    servos.insert("ZL", LedcDriver::new(peripherals.ledc.channel5, &ledc_timer, peripherals.pins.gpio38).unwrap());
    servos.insert("CU", LedcDriver::new(peripherals.ledc.channel6, &ledc_timer, peripherals.pins.gpio37).unwrap());
    servos.insert("CL", LedcDriver::new(peripherals.ledc.channel7, &ledc_timer, peripherals.pins.gpio36).unwrap());

    Robot{ i2c, uart, mpu, sht, servos }
}

fn main() {
    let robot = startup();

    let Robot { mut i2c, mpu, sht, uart: uart_sensor, mut servos } = robot;
    let uart_command = Arc::clone(&uart_sensor);

    // Sensor Info
    thread::spawn(move || {
        FreeRtos::delay_ms(5000);

        let mut robot_readings = RobotReadings{ position: None, weather: None };
        loop {
            FreeRtos::delay_ms(5000);
            log::info!("Sensor Thread Loop");
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
        loop {
            log::info!("Execute Thread Loop");
            let command = { uart::read_from_uart(&uart_command) };
            match command {
                Some(command) => {
                    match commands::parse_command(&command, &mut servos) {
                        Some(mut function) => {
                            uart::write_to_uart(&mut uart_command.lock().unwrap(), (OK.to_string() + ENDLINE).as_str());
                            if let Err(err) = function() {
                                log::error!("{:?}", err);
                            }
                        },
                        None => uart::write_to_uart(&mut uart_command.lock().unwrap(), (ERR.to_string() + ENDLINE).as_str()),
                        }
                },
                None => FreeRtos::delay_ms(100),
            }
        }
    }); 

    loop {
        FreeRtos::delay_ms(100000);
    }
}
