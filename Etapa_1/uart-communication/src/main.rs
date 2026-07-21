use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::uart::{UartDriver, config::Config};
use esp_idf_svc::hal::units::Hertz;
use std::fmt::Write;

fn main() {
    esp_idf_svc::sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let config = Config::default().baudrate(Hertz(115_200));
    let mut uart: UartDriver = UartDriver::new(
        peripherals.uart1,
        pins.gpio1,
        pins.gpio2,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config,
    ).unwrap();
    let x = 9;
    write!(uart, "Hello!\n").unwrap();
    write!(uart, "Goodbye!\n").unwrap();
    write!(uart, "Value of x is: {x}\n").unwrap();
    writeln!(uart, "Another way to print!").unwrap();
}