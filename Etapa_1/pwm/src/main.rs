use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver}; // Modulos para la configuracion del PWM
use esp_idf_svc::hal::units::FromValueType; // Permite utilizar unidades como kHz(), MHz(), Hz(), etc.
use esp_idf_svc::hal::delay::FreeRtos;
use embedded_hal::pwm::SetDutyCycle; // Para controlar directamente el Duty cycle del pin output

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default().frequency(25u32.kHz().into())).unwrap(); // Inicializa el temporizador del periférico LEDC con una frecuencia de 25 kHz.
    let mut pin_rojo = LedcDriver::new(peripherals.ledc.channel0, timer_driver, pins.gpio18).unwrap(); // Crea un canal PWM utilizando el temporizador previamente configurado y lo asocia al pin 18
    loop {
        for i in 0u8..=100{
            pin_rojo.set_duty_cycle_percent(i).unwrap(); // Actualiza el duty cycle del PWM utilizando un porcentaje entre 0 % y 100 %.
            log::info!("Duty cycle: {i}");
            FreeRtos::delay_ms(50);
        }
    }
}