## Guía de Funcionamiento

### Descripción
Esta práctica consistió en encender un diodo LED (rojo) utilizando los pines de una placa ESP32-S3 y un transistor BJT NPN 2N2222. Con esto se busca explicar los conceptos básicos del funcionamiento de Rust en arquitecturas de microcontroladores de la familia ESP32.

Los planos de conexiones esquemáticas, el diseño de PCB y una fotografía del circuito en la protoboard se encuentran en la carpeta `Hardware/`.

### Herramientas
- Placa de desarrollo ESP32 (para esta práctica se utilizó el modelo S3-N16R8).
- Un diodo LED (esta práctica utilizó uno de color rojo).
- Un transistor BJT NPN 2N2222.
- Una resistencia de 2.2kΩ.
- Una resistencia de 330Ω.
- Los cables necesarios para las conexiones.

## Código
### Importación de módulos y funciones
Este código opera utilizando `esp_idf_svc`, que es la librería principal de Espressif para proyectos que involucran la versión std de Rust en ESP32. Dentro de este crate (biblioteca) se utiliza el módulo `HAL` (Hardware Abstraction Layer). El funcionamiento de `HAL` es actuar como traductor de funciones complejas del SDK de C nativo de la ESP32 a tipos de datos y estructuras idiomáticas de Rust.
Una vez cargada la capa de abstracción, se cargaron los módulos específicos a utilizar.

**Módulos utilizados:**
- `peripherals` = módulo para gestión global del chip
- `gpio` = módulo para entradas y salidas digitales
- `delay` = módulo para el control de tiempo

Cada módulo incluye su propia función interna para trabajar el propósito por el cual fue incluido:
- `peripherals -> Peripherals`
- `gpio -> PinDriver`
- `delay -> FreeRtos`
```rust
use esp_idf_svc::hal::{peripherals::Peripherals, gpio::PinDriver, delay::FreeRtos};
```
Es la línea encargada de unificar todo lo mencionado.

### Parche del C-SDK
Como se mencionó arriba, al ser un sistema std, depende de una capa de abstracción de las funciones críticas del SDK de C de la ESP. Para "inyectar" este parche se utiliza la línea:
```rust
esp_idf_svc::sys::link_patches();
```
Esta se encarga de indicarle al compilador que incluya y referencie los parches del C-SDK dentro del binario final tras compilar el proyecto. Esto permite evitar reinicios, panics o comportamientos inesperados/impredecibles dentro del microcontrolador.

### Impresión en consola
Pese a ser opcional y no indispensable, se incluyó una función para imprimir en la terminal cuando el LED esté encendido o apagado. Para poder utilizar esta herramienta de logs, se debe llamar de la biblioteca principal de Espressif la función correspondiente que habilita el uso de logs en terminal, la cual es:
```rust
esp_idf_svc::log::EspLogger::initialize_default();
```
Si se utilizan las funciones de log que se presentarán más adelante en esta explicación, pero no se llama esta función, el código no fallará al compilarse, pero los mensajes no aparecerán en la terminal.

### Control de periféricos
Para que el código funcione adecuadamente con la ESP32, se debe contar con el control de todos sus periféricos. Para lograr esto se utiliza la función:
```rust
let peripherals = Peripherals::take().unwrap();
```
La cual crea una variable de nombre `peripherals` y puede recibir dos valores:
- Si todo sale bien, se le da acceso a **todos los elementos** con los que cuenta la ESP32, luciendo así: `Result::Ok(Peripherals)`.
- Si algo sale mal, recibe un `Err` con la informacion del error (`Result::Err(EspError)`). Por lo general algo sale mal cuando otra variable ya tiene control de los periféricos, y para evitar estos errores, se frena aquí el código. Como dato adicional, una vez que el programa recibe un `Err`, este entra en **panic** y se detiene la ejecución entrando en panic.

Del mismo modo, esta función de retorno de valor se divide en dos partes:
- **Asignación**: `Peripherals::take()` recibe o no el control.
- **Control de error**: `.unwrap()`, en caso de que la operación haya fallado y regrese `None`, se detiene el programa para evitar comportamientos inesperados. No es forzoso utilizar esta función; se puede eliminar esta parte y el código seguiría funcionando, siempre y cuando se implemente manualmente una secuencia para el manejo de errores, aunque esto cambiaría la estructura completa de la función en este apartado.

### Asignación de periféricos
Una vez que se tiene control de los periféricos del microcontrolador, queda asignarlos a variables encargadas de operar con ellos. Para eso utilizamos esta línea:
```rust
let mut pin = PinDriver::output(peripherals.pins.gpio5).unwrap();
```
Nótese que la variable forzosamente tiene que ser mutable, ya que los estados que pueda tener el periférico indican cambios en su comportamiento, cambios que una variable inmutable no podría ofrecer sin entrar en panic. Para este ejemplo, la variable que opera el pin se llama `pin`.
Del mismo modo, esta función de retorno de valor también se divide en dos partes:
- Para **controlar un solo** periférico: `PinDriver::output(peripherals.pins.gpio5)`; en este proyecto se utilizó el pin 5. Es importante aclarar que, dependiendo del modelo, varía la cantidad de pines y, con ello, la función que tienen. Algunos pines de microcontroladores ESP32 tienen propósitos únicos, por lo que se recomienda revisarlos antes de asignarlos.
- **Control de error**: `.unwrap()`, mismo escenario que en el apartado anterior; en caso de presentarse un error con la asignación del pin, el programa entra automáticamente en panic para evitar comportamiento indefinido.

### Bucle central
En este apartado se evaluará el flujo que existe dentro del `loop` del código:
- Encender el LED: `pin.set_high().unwrap();`
- Imprimir en la terminal: `log::info!("LED encendido");`. Recuerda que para utilizar esto debiste llamar la función que se encuentra en el apartado **Impresión en consola**.
- Esperar 1 segundo: `FreeRtos::delay_ms(1000);`. Nótese que el tiempo está en milisegundos.
- Apagar el LED: `pin.set_low().unwrap();`