use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::Pull;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::TickType_t;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
 
    log::info!("Hello, world!");
    FreeRtos::delay_ms(5000);

    let peripherals = Peripherals::take().unwrap();
    let green_button = PinDriver::input(peripherals.pins.gpio7, Pull::Up).unwrap();
    
    log::info!("Survived the panic");

    let config = I2cConfig::new().baudrate(Hertz(60));
    let sda = peripherals.pins.gpio8;
    let scl = peripherals.pins.gpio9;
    let mut value = [0u8; 1];

    let mut i2c_driver = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    loop {
        match i2c_driver.write_read(0x68u8, &[0x75u8], &mut value, 800u32) {
            Ok(_) => log::info!("I AM: {}", value[0]),
            Err(e) => log::error!("I2C error: {:?}", e),
        }
    }

    // loop {
    //     if green_button.is_low() {
    //         log::info!("Happy happy happy!!!");
    //         FreeRtos::delay_ms(500);
    //     }
    //     FreeRtos::delay_ms(500);
    // }
    
}
