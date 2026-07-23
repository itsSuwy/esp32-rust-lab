use esp_idf_svc::hal::gpio::{PinDriver, Pull, Input, Output};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    // A continuacion se listan 3 variantes de pull para la variable boton
    //let button = PinDriver::input(pins.gpio17, Pull::Floating).unwrap(); // Variante 1 <- Depende de hardware externo
    let button = PinDriver::input(pins.gpio17, Pull::Down).unwrap(); // Variante 2 <- Se utiliza el protocolo interno del ESP32 para controlar el Pull Down
    //let button = PinDriver::input(pins.gpio17, Pull::Up).unwrap(); // Variante 3 <- Se utiliza el protocolo interno del ESP32 para controlar un Pull up
    let mut led = PinDriver::output(pins.gpio18).unwrap();
    func_pull_down(&button, &mut led); // Funcion exclusiva para protocolos pull down
    //func_pull_up(&button, &mut led); // Funcion exclusiva para protocolos pull up
}

// Funciones exclusivas para variantes 1 y 2 (Pull down por hardware o pull down por software)
fn func_pull_down(button: &PinDriver<Input>, led: &mut PinDriver<Output>){ // Se encarga de ejecutar la logica completa del Pull-Up
    loop{
        if boton_control_down(&button){ // Evalua si el boton fue presionado o no
            log::info!("Pulsacion confirmada!");
            led.set_high().unwrap();
            wait_for_release_down(&button); // Filtra rebotes al dejar de oprimir el boton
            led.set_low().unwrap();
        }
        FreeRtos::delay_ms(50);
    }
}
fn boton_control_down(boton: &PinDriver<Input>) -> bool{ // Evalua si el boton esta siendo presionado
    if !boton.is_high() {
        return false; // ni siquiera empezó a presionarse
    }
    FreeRtos::delay_ms(20);
    boton.is_high() // confirma si sigue presionado tras el delay
}
fn wait_for_release_down(boton: &PinDriver<Input>) { // Filtra posibles rebotes
    FreeRtos::delay_ms(100);
    while !boton.is_low(){
        FreeRtos::delay_ms(5);
    }
}

// Funciones exclusivas para variante 3 (Pull up)
fn func_pull_up(button: &PinDriver<Input>, led: &mut PinDriver<Output>){ // Logica principal del pull up
    loop{
        if !boton_control_up(&button){ // Retorna false cuando el boton este presionado
            log::info!("Boton pulsado!");
            led.set_high().unwrap();
            wait_for_release_up(&button);
            led.set_low().unwrap();
        }
        FreeRtos::delay_ms(50);
    }
}

fn boton_control_up(boton: &PinDriver<Input>) -> bool{
    if !boton.is_low() {
        return false; // ni siquiera empezó a presionarse
    }
    FreeRtos::delay_ms(20);
    boton.is_low() // confirma si sigue presionado tras el delay
}

fn wait_for_release_up(boton: &PinDriver<Input>) { // FIltra posibles rebotes
    FreeRtos::delay_ms(100);
    while !boton.is_high(){
        FreeRtos::delay_ms(5);
    }
}