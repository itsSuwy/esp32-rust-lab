use esp_idf_svc::hal::{peripherals::Peripherals, gpio::PinDriver, delay::FreeRtos};
fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut pin = PinDriver::output(peripherals.pins.gpio5).unwrap();

    loop {
        pin.set_high().unwrap();
        log::info!("LED encendido");
        FreeRtos::delay_ms(1000);

        pin.set_low().unwrap();
        log::info!("LED apagado");
        FreeRtos::delay_ms(1000);
    }
}