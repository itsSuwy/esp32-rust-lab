// Importaciones realizadas
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::AnyIOPin; // Para convertir los pines en genericos
use esp_idf_svc::hal::uart::{UartDriver, config::Config}; // Modulo central para la comunicacion
use esp_idf_svc::hal::units::Hertz;
use std::fmt::Write; // Indispensable para aplicar formato de impresion

fn main() {
    esp_idf_svc::sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins; // Se guarda en una variable inmutable la configuracion de perififericos
    let config = Config::default().baudrate(Hertz(115_200)); // Crea un struct de configuracion modificando su frecuencia interna a 115_220
    let mut uart: UartDriver = UartDriver::new(
        peripherals.uart1, // Se reclama el periferico UART1
        pins.gpio1, // Se le asigna funcionamiento de TX
        pins.gpio2, // Se le asigna funcionamiento de RX
        Option::<AnyIOPin>::None, // Sin RTS
        Option::<AnyIOPin>::None, // Sin  CTS
        &config, // Se aplican los parametros asignados en la variable config
    ).unwrap();
    let x = 9;
    write!(uart, "Hello!\n").unwrap(); // Forma uno para escribir en UART
    write!(uart, "Value of x is: {x}\n").unwrap();
    writeln!(uart, "Another way to print!").unwrap(); // Forma dos para escribir en UART
}