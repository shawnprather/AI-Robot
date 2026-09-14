use esp_idf_svc::hal::ledc::LedcDriver;
use esp_idf_svc::hal::sys::EspError;

// Other
const PERIOD_LENGTH: f32 = 20.0; 

pub fn set_angle(servo: &mut LedcDriver, angle: f32) -> Result<(),EspError> {
    let clamped_angle = angle.clamp(0.0, 180.0);
    let time_per_burst: f32 = (clamped_angle * (1.0/90.0)) + 0.5;
    let ratio_on = time_per_burst/PERIOD_LENGTH;
    servo.set_duty((ratio_on * servo.get_max_duty() as f32) as u32)?;
    Ok(())
}