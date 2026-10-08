use std::collections::HashMap;

use esp_idf_svc::{hal::ledc::LedcDriver, sys::EspError};

use crate::servos::mg90s;

pub const SERVO: &str = "SERVO";

pub const OK: &str = "OK";
pub const ERR: &str = "ERR";

pub const ENDLINE: &str = "\n";

pub fn parse_command<'a>(command: &'a str, servos: &'a mut HashMap<&'static str, LedcDriver<'static>>)
    -> Option<Box<dyn FnMut() -> Result<(), EspError> + 'a>> {
    let mut args= command.trim().split_whitespace();
    match args.next() {
        Some(SERVO) => {
            let servo = servos.get_mut(args.next()?)?;
            let angle = args.next()?.parse::<f32>().ok()?;

            Some(Box::new(move || { 
                mg90s::set_angle(servo, angle)
            }))
        },
        _  => None
    }
}