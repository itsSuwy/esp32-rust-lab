use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution}; // Modulos para la configuracion del PWM
use esp_idf_svc::hal::units::FromValueType; // Permite utilizar unidades como kHz(), MHz(), Hz(), etc.
use esp_idf_svc::hal::delay::FreeRtos;
use embedded_hal::pwm::SetDutyCycle; // Para controlar directamente el Duty cycle del pin output

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let timer_driver = LedcTimerDriver::new( // Inicializamos un temporizador del periférico LEDC
        peripherals.ledc.timer0, // El timer que se utilizara
        &TimerConfig::new() // Se modifican los parametros para la configuracion del Timer
            .frequency(38u32.kHz().into())
            .resolution(Resolution::Bits11))
        .unwrap();
    // let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0,&TimerConfig::default() .frequency(25u32.kHz().into()).resolution(Resolution::Bits11)).unwrap(); // De manera fancy se puede escribir asi tambien
    let mut pin_rojo = LedcDriver::new( // Crea un canal PWM utilizando el temporizador previamente configurado y lo asocia al pin 18
        peripherals.ledc.channel0,
        timer_driver,
        pins.gpio18)
        .unwrap();
    loop {
        for i in 0u8..=100{
            pin_rojo.set_duty_cycle_percent(i).unwrap(); // Actualiza el duty cycle del PWM utilizando un porcentaje entre 0 % y 100 %.
            log::info!("Duty cycle increasing: {i}");
            FreeRtos::delay_ms(50);
        }
        for i in (0u8..=100).rev(){
            pin_rojo.set_duty_cycle_percent(i).unwrap();
            log::info!("Duty cycle decreasing: {i}");
            FreeRtos::delay_ms(50);
        }
        FreeRtos::delay_ms(50);
    }
}