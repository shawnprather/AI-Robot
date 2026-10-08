use esp_idf_svc::{hal::{delay::FreeRtos, uart::UartDriver}, sys::ESP_ERR_TIMEOUT};
use std::sync::{Mutex, Arc};

const MAX_PRINT_LENGTH: usize = 50;

pub fn write_to_uart(uart: &mut UartDriver, string: &str) {
    log::info!("Writing To UART: {}", to_display_string(string, MAX_PRINT_LENGTH));
    if let Err(e) = uart.write(string.as_bytes()) {
        log::error!("Uart write failed: {:?}", e);
    }
}

pub fn read_from_uart(uart: &Arc<Mutex<UartDriver>>) -> Option<String> {
    let mut string = String::with_capacity(128);
    loop {
        let mut byte = [0u8; 1];
        let result = {
            uart.lock().unwrap().read(&mut byte, 0)
        };
        match result {
            Ok(1) => {
                if byte[0] == b'\n' {
                    log::info!("Command: {}", string);
                    return Some(string)
                } else if !(byte[0] as char).is_ascii() {
                    return None
                }
                string.push(byte[0] as char);
            }
            Ok(_) => {
                if string.is_empty() { return None; }
                FreeRtos::delay_ms(5);
            }
            Err(e) if e.code() == ESP_ERR_TIMEOUT => {
                if string.is_empty() { return None; }
                FreeRtos::delay_ms(5);
            }
            Err(e) => {
                log::error!("UART read failed: {:?}", e);
                return None;
            }
        }
    }
}

fn to_display_string(string: &str, max_length: usize) -> &str {
    let trimmed_string = string.trim();
    return &trimmed_string[..trimmed_string.len().min(max_length)]
}