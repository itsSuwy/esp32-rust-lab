use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver};
use esp_idf_svc::hal::units::FromValueType;
use esp_idf_svc::hal::delay::FreeRtos;
use embedded_hal::pwm::SetDutyCycle;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default().frequency(25u32.kHz().into())).unwrap();
    let mut pin_rojo = LedcDriver::new(peripherals.ledc.channel0, timer_driver, pins.gpio18).unwrap();
    loop {
        for i in 0u8..=100{
            pin_rojo.set_duty_cycle_percent(i).unwrap();
            log::info!("Duty cycle: {i}");
            FreeRtos::delay_ms(50);
        }
    }
}