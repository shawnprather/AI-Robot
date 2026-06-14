use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::Pull;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::units::Hertz;

mod sensors;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
 
    log::info!("Powered On");
    FreeRtos::delay_ms(1000);

    let peripherals = Peripherals::take().unwrap();
    //let green_button = PinDriver::input(peripherals.pins.gpio7, Pull::Up).unwrap();

    let config = I2cConfig::new().baudrate(Hertz(60));
    let sda = peripherals.pins.gpio8;
    let scl = peripherals.pins.gpio9;
    let mut i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    match sensors::mpu6050::wake(&mut i2c_driver) {
        Ok(_) => log::info!("Woke"),
        Err(e) => log::error!("I2C MPU Wake Error: {:?}", e)
    }

    match sensors::mpu6050::who_am_i(&mut i2c_driver) {
        Ok(val) => log::info!("I AM: {}", val),
        Err(e) => log::error!("I2C error: {:?}", e),
    }

    loop {
        match sensors::mpu6050::get_accel_data(&mut i2c_driver) {
            Ok(accel) => log::info!("Accel: [{}] [{}] [{}]", accel[0], accel[1], accel[2]),
            Err(e) => log::error!("I2C error, get accel: {:?}", e),
        }

        match sensors::mpu6050::get_gyro_data(&mut i2c_driver) {
            Ok(gyro) => log::info!("Gyro: [{}] [{}] [{}]", gyro[0], gyro[1], gyro[2]),
            Err(e) => log::error!("I2C error, get gyro: {:?}", e),
        }
    }
    
}
