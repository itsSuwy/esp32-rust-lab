use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver, config::AdcChannelConfig};
use esp_idf_svc::hal::adc::attenuation::DB_12;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let adc = AdcDriver::new(peripherals.adc1).unwrap(); // Asignamos un pin de ADC
    let adc_config = AdcChannelConfig{
        attenuation: DB_12,
        ..Default::default()
    }; // Creamos un bloque de configuracion default
    let mut adc_pin = AdcChannelDriver::new( // Asignamos la configuracion a un pin de salida
        &adc, // El adc interno
        pins.gpio7, // pin de salida
        &adc_config).unwrap(); // bloque de configuracion junto al manejo de errores

    loop{
        log::info!("ADC value: {}", adc.read(&mut adc_pin).unwrap());
        FreeRtos::delay_ms(100);
    }
}