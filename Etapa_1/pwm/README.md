## Guia de Funcionamiento
### Descripcion
Esta práctica consistió en la implementación del periférico de control de LEDs (LEDC) del microcontralor ESP32-S3 utilizando el protocolo PWM para controlar de manera continua y progresiva la intensidad luminica de un diodo led a una frecuencia de 38kHz. La práctica abarca la configuración del temporizador (timer) del periférico y la asignación del canal de salida.

### Herramientas
- Diodo led rojo
- Transistor NPN 2N2222
- Resistencia de 2.2kΩ.
- Resistencia de 330Ω.
- Placa de desarrollo ESP32 (esta práctica utilizó el modelo S3-N16R8).
- Cables necesarios para las conexiones.

### ¿Que es el protocolo PWM?
La Modulación por Ancho de Pulso (PWM, por sus siglas en inglés: Pulse-Width Modulation) es una técnica empleada para simular una señal analógica continua a partir de una salida digital que solo puede tomar dos estados: HIGH (VCC) o LOW (GND).  
En lugar de variar el voltaje de salida del pin (que en el ESP32-S3 es de 3.3 V fijos), el microcontrolador conmuta la señal entre encendido y apagado a una frecuencia tan alta que el componente o la vista humana no perciben el parpadeo, sino un valor promedio de potencia.  
Los parámetros fundamentales que definen una señal PWM son:
- **Frecuencia (_f_)**: Define qué tan rápido se repite el ciclo completo de encendido/apagado por segundo, medido en Hertz. En esta práctica se configuró a 38 kHz (38,000 ciclos por segundo). Es importante aclarar que Frecuencias bajas provocarian que el ojo humano perciba al Led en un estado "parpadeante" en lugar de una "atenuacion continua".
- **Periodo (_T_)**: Es la duración de un ciclo completo, calculado como $T = \frac{1}{f}$. Para 38 kHz, el periodo es de 26.31578 microseconds ($\mu s$).
- **Ciclo de trabajo (_Duty Cycle_)**: El porcentaje de tiempo dentro de un periodo en el cual la señal permanece en estado HIGH (1 lógico).
- - 0% : La señal está completamente apagada (0V promedio).
- - 50% : La señal pasa la mitad del tiempo encendida y la otra mitad apagada.
- - 100% : La señal está completamente encendida (3.3V promedio).
- Resolucion (R) : Define la cantidad de pasos discretos en los que se puede dividir un ciclo completo para ajustar el duty cycle. Resoluciones bajas (4 bits / 16 pasos) ocasionan parpadeos / saltos de estado perceptibles para el ojo humano, en cambio, resoluciones mas altas (10 bits / 1024 pasos) logran cambios mucho mas fluidos y tenues para el ojo.

#### Relacion Resolucion-Frecuencia
El hardware LEDC opera con un reloj base fijo de cierta velocidad maxima, el cual se reparte entre frecuencia y resolucion. Si se quisieran más pasos de resolución dentro de cada ciclo, cada ciclo necesita "caber" más divisiones de ese reloj base, lo que limita qué tan rápido puede repetirse el ciclo completo.  
- **Resolucion alta + Frecuencia alta** = Lo ideal para trabajar con Leds pero el reloj de la ESP32 sera una limitacion de hardware.
- **Resolución alta + Frecuencia baja** = Es posible, pero puede caer en rango de parpadeo perceptible.
- **Resolución baja + Frecuencia alta** = Posible, pero se pierde finura de control.

## Codigo
### Importación de módulos y funciones
Ademas de los modulos conocidos que ya se han utilizado en practicas anteriores, esta practica conto con el uso de estos nuevos crates:
```rust
use esp_idf_svc::hal::ledc::{...};
use embedded_hal::pwm::{...};
```
`ledc` Es el módulo específico para el ESP32. Se encarga de interactuar directamente con el periférico de hardware LEDC para gestionar el control, la configuración del reloj y la asignación física de la señal PWM en el microcontrolador.  
Por otra parte `pwm` opera como una abstraccion estandar del ecosistema de Rust para sistemas embebidos, no maneja el hardware directamente, sino que define métodos universales para modificar el ciclo de trabajo de la señal de forma estandarizada y portable.   
Notese que `pwm` proviene del modulo `embedded_hal` el cual a diferencia de `esp_idf_svc` (Enfocado en arquitectura de ESP32), esta orientado a una serie de parametros genericos que pueden servir en cualquier microcontrolador: ESP32, STM32, ATmega, etc.

### Creacion del bloque de configuracion
Para operar el protocolo PWM del ESP32 primero debemos configurar tres parametros internos los cuales son:
- Indicar cual reloj interno del microcontrolador se debera utilizar.
- Asignar una frecuencia
- Asignar una resolucion.

Por ello creamos la siguiente variable encargada de guarda la configuracion
```rust
let timer_driver = LedcTimerDriver::new(...)
```
Notese que se esta utilizando la funcion `new` proveniente de `LedcTimerDriver` la cual internamente funciona de la siguiente manera
```rust
pub fn new<T: LedcTimer<SpeedMode = S> + 'd>(
    _timer: T,
    config: &TimerConfig,
) -> Result<Self, EspError>
```
Espera recibir como parametro un reloj interno del microcontrolador `_timer: T` asi como un bloque de configuracion para el reloj `config: &TimerConfig` y al recibir ambos parametros, devuelve un resultado: ya sea el bloque generado o un error `Result<Self, EspError>`.  
Dentro de este codigo, se asigno el reloj numero 0 del ESP32 `new`
```rust
let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::new()
        ...
```
Notese que la configuracion del timer se trabaja de manera **new** debido a que TimerConfig internamente se representa mediante un registro (struct) con la siguiente sintaxis:
```rust
pub struct TimerConfig {
    pub frequency: Hertz,
    pub resolution: Resolution,
}
```
Entonces la configuracion del Timer debe asignarle valores a dos campos: Resolucion (en Bits) y Frecuencia (en Hertz).  
Por ello el campo cambia y se utiliza la funcion new que espera recibir explicitamente los valores para el registro interno  
```rust
    ...
        &TimerConfig::new() // Se modifican los parametros para la configuracion del Timer
            .frequency(38u32.kHz().into())
            .resolution(Resolution::Bits11))
        ...
```
Nótese como aclaración importante que se debe mantener una relación estricta entre la resolución y la frecuencia. Dado que en esta práctica se utilizó una frecuencia de 38 kHz, la resolución debía ser menor a 12 bits; de lo contrario, el reloj interno del ESP32-S3 sería incapaz de generar la señal por un límite físico del hardware.

Otro camino para manejar los campos de frecuencia y resolución sería utilizar únicamente `&TimerConfig::default()`, ya que asigna valores genéricos y seguros para el microcontrolador que evitan provocar un *panic* durante la ejecución.

Para determinar la resolución y la frecuencia máximas soportadas por el microcontrolador nos apoyamos en la siguiente fórmula:

$$f_{PWM\_max} = \frac{f_{clock}}{2^N}$$

Donde:
- **$f_{clock}$**: Frecuencia del reloj base interno (comúnmente 80 MHz para el reloj APB del ESP32).
- **$N$**: Resolución de la señal expresada en bits (donde $2^N$ representa la cantidad total de niveles o pasos del *duty cycle*).
- **$f_{PWM\_max}$**: Frecuencia máxima alcanzable para la resolución $N$ elegida.

En conclusión, si la frecuencia resultante ($f_{PWM\_max}$) es mayor o igual que la frecuencia con la que se desea trabajar ($f_{deseada}$), el microcontrolador podrá generar la señal sin inconvenientes. Caso contrario, si la frecuencia deseada supera a la máxima permitida por el hardware, el driver lanzará un *panic* al intentar inicializar la configuración. Para solucionarlo, se deberia disminuir la cantidad de pasos ($N$) para entrar en un margen aceptable.  
  
Por ultimo, el bloque de configuracion para `PWM` de esta practica lucio asi:
```rust
let timer_driver = LedcTimerDriver::new( 
        peripherals.ledc.timer0, 
        &TimerConfig::new() 
            .frequency(38u32.kHz().into())
            .resolution(Resolution::Bits11))
        .unwrap();
```

### Creacion y asignacion del canal PWM
Una vez completado el bloque de configuracion con los parametros con los que el microcontrolador interactuara internamente con su reloj en el `PWM` ahora queda usar esa configuracion en una variable y asignarla a un Pin especifico. Aqui es cuando entra la siguiente linea:  
```rust
let mut pin_rojo = LedcDriver::new(...)
```
Notese que se llama a un registro de nombre `LedcDriver` el cual internamente luce asi:
```rust
pub struct LedcDriver<'d> {
    channel: u8,
    timer: u8,
    duty: Duty,
    hpoint: HPoint,
    speed_mode: ledc_mode_t,
    max_duty: Duty,
    _p: PhantomData<&'d mut ()>,   
}
```
No obstante al llamarse con la funcion `new()`, indica una creacion de cero esperando los siguientes parametros:
```rust
pub fn new<C, B>(
    _channel: C,
    timer_driver: B,
    pin: impl OutputPin + 'd,
) -> Result<Self, EspError>
```
La cual espera recibir un: 
- Canal de pwm del microcontrolador.
- La configuracion del pwm escrita previamente.
- Un pin de salida.  

Una vez satisfechos los requerimientos, la funcion devuelve un estado ya sea de error o a si mismo en caso de exito.

Por ello en el codigo luce de esta manera:
```rust
  let mut pin_rojo = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        pins.gpio18)
        .unwrap();
```
La variable mutable de nombre `pin_rojo` esta recibiendo el canal 0 de reloj, nuestra configuracion de la variable timer_driver y el pin de salida numero 18.

### Control del ciclo de trabajo (Duty-cycle)
Como se menciono en el inicio de esta explicacion, ademas de la resolucion y frecuencia, el `PWM` depende de un control en su duty-cycle para alterar el pulso que reciba el componente electrico conectado al pin de salida (Led, motor, etc).
Y aqui es cuando entra el segundo modulo utilizado `embedded_hal::pwm`, al ser un trait generico, cualquier microcontrolador puede utilizar el control hacia la salida PWM. Es por ello que se utilizo la siguiente funcion:
```rust
fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), Self::Error>
```
La cual toma un numero en el rango de 0 a 100 (incluyendo extremos) para asignarle automaticamente un porcentaje, evitando teniendo que calcular manualmente el duty-cycle de cada iteracion.  
Esta funcion se trabajo de dos maneras dentro del loop principal:
- De manera **Ascendente**: de 0 a 100.
- De manera **Descendente** : de 100 a 0.   

Para visualizar el encendido y apagado de un led mediante PWM.  
```rust
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
```
Notese que se utilizaron delays ligeros (50 milisegundos) entre cada porcentaje del duty-cycle del `PWM`.