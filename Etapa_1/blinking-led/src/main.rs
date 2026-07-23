// Modulos y funciones utilizadas
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::delay::FreeRtos;
fn main() {
    esp_idf_svc::sys::link_patches(); // Se aplica el parche de C-SDK
    esp_idf_svc::log::EspLogger::initialize_default(); // Se inicializa el entorno para usar logs
    let peripherals = Peripherals::take().unwrap(); // Se toma el control de los perifericos del microcontrolador
    let pins = peripherals.pins;
    let mut led = PinDriver::output(pins.gpio5).unwrap(); // Se asigna el pint 5 a una variable de caracter mutable

    loop {
        led.set_high().unwrap(); // Se enciende el led
        log::info!("LED encendido");
        FreeRtos::delay_ms(500);

        led.set_low().unwrap(); // Se apaga el led
        log::info!("LED apagado");
        FreeRtos::delay_ms(500); // Delay de 500 ms
    }
}