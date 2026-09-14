use esp_idf_svc::{hal::uart::UartDriver, sys::ESP_ERR_TIMEOUT};

pub fn write_to_uart(uart: &mut UartDriver, string: &str) {
    if let Err(e) = uart.write(string.as_bytes()) {
        log::error!("Uart write failed: {:?}", e);
    }
}

pub fn read_from_uart(uart: &mut UartDriver) -> Option<String> {
    let mut string = String::with_capacity(128);
    loop {
        let mut byte = [0u8; 1];
        match uart.read(&mut byte, 60) {
            Ok(1) => {
                if byte[0] == b'\n' {
                    return Some(string)
                }
                string.push(byte[0] as char);
            }
            Ok(_) => continue,
            Err(e) if e.code() == ESP_ERR_TIMEOUT => continue,
            Err(e) => {
                log::error!("UART read failed: {:?}", e);
                return None
            }
        }
    }
}