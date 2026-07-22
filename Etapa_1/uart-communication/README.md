## Guia de funcionamiento
### Descripcion
Esta practica consistio en implementar comunicacion UART basica dentro del microcontrolador.  
El UART hace referencia a comunicacion serial punto a punto sin reloj compartido siendo indispensable cuando se busca realizar herramientas de debug dentro del microcontrolador.   

## Codigo
### Importación de módulos y funciones
**Modulos utilizados**
- `uart` = Modulo fundamental para la comunicacion del proyecto. _# Dependencia de HAL_
- `units` = Modulo de control de unidades. _# Dependencia de HAL_
- `std` = Modulo estandar de Rust llama a `fmt` para poder controlar impresiones en UART _(Este modulo es nativo de rust estandar)_.

Notese que se omitieron modulos que ya han sido utilizados en proyectos anteriores.  
**Funciones utilizadas de los modulos**
- `AnyIOPin` = Proveniente de `gpio` se encarga de envolver la estructura de los tipos de pin gpio, convirtiendolos en tipos genericos y unificados.
- `UartDriver` = Toma el control fisico de los perifericos asignados y los configura.
- `Config` = Struct que contiene datos de configuracion, no toca el hardware por si solo, se utiliza para definir las reglas y parametros de transmision antes de encender el canal. Para este ejemplo se utilizo en el Baud Rate en la comunicacion.

### Asinacion de perifericos
Utilizando un enfoque diferente, se decidio guardar la configuracion de periferico en una variable llamada `pins` para evitar tener que escribir `peripherals.pins` a cada pin que se utilice.
```rust
let pins = peripherals.pins;
//Logrando pasar de esto:
peripherals.pins.gpio5 // Uso del pin 5
// A esto:
pins.gpio5 // Uso del pin 5
```
Ambas son completamente validas, es cuestion de simplicidad en escritura.

### Creacion de un bloque de configuracion
Internamente la comunicacion UART requiere seguir un registro para modificar su propio campo de configuracion interna. Dicho registro maneja multiples campos: mode, baudrate, parity, etc.  
Es por ello que llamamos a una variable inmutable de configuracion que almacene estos parametros para pasarsela por referencia al UART.
```rust
let config = Config::default().baudrate(Hertz(115_200));
```
Notese que `Config` hace referencia a un struct de configuracion interno dentro del codigo que luce exactamente asi:  
```rust
pub struct Config {
    pub mode: Mode,
    pub baudrate: Hertz,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
    pub flow_control_rts_threshold: u8,
    pub source_clock: SourceClock,
    pub intr_flags: EnumSet<InterruptType>,
    pub event_config: EventConfig,
    pub rx_fifo_size: usize,
    pub tx_fifo_size: usize,
    pub queue_size: usize,
}
```
Por ende cuando utilizamos `Config::default`, estamos manejando una configuracion estandar interna dentro del struct.  
No obstante, la comunicacion UART es asincrona (sin reloj compartido entre transmisor y receptor), ambos dispositivos deben coincidir con el mismo baud rate para interpretar cuando empieza y termina cada bit, es por ende que tenemos dos caminos que se apegan a lo que necesitemos:  
- **Opcion 1**: Utilizar la configuracion por default.
- **Opcion 2**: Modificar el campo a uno que querramos.

Durante esta practica se opto por modificar el baud rate a uno de 115200 baudios
```rust
.baudrate(Hertz(115_200)); // El guion es solo para mejorar legibilidad
```

### Configuracion UART 
Para implementar comunicacion UART se llama a la funcion `UartDriver::new()`, la cual internamente luce asi:
```rust
pub fn new<UART: Uart + 'd>(
uart: UART,
tx: impl OutputPin + 'd,
rx: impl InputPin + 'd,
cts: Option<impl InputPin + 'd>,
rts: Option<impl OutputPin + 'd>,
config: &config::Config,
)
```
Desglosandolo cada campo trabajaria de la misma forma  
- **Asignarle un controlador UART**  
Depende del modelo del chip la cantidad de controladores disponibles.
```rust
uart: UART, 
```
- **Asignar pines _transmisores (TX)_ y _receptores_ (RX)**  
```rust
tx: impl OutputPin + 'd,
rx: impl InputPin + 'd,
```
- **Asignar pines de control para envio de informacion**  
Aqui entran dos conceptos:
- - **RTS (Request to send)**: El cual le indica al dispositivo **destino** que el **origen** tiene informacion para poder trasmitirle y solicita permiso para enviarla.
- - **CTS (Clear to send)**: El dispositivo destino da autorizacion --o no-- al origen para recibir su informacion.  

En codigo luce asi:
```rust
cts: Option<impl InputPin + 'd>,
rts: Option<impl OutputPin + 'd>,
```
Notese que en el codigo de la practica estos valores fueron asignados como `Option::<AnyIOPin>::None,`, indicando que no se utilizarian.
- **Parametros de configuracion**  
La comunicación UART requiere que se le indiquen los parámetros de configuración antes de inicializar el canal. Por eso el constructor `UartDriver::new()` recibe el `Config` ya armado como uno de sus argumentos:
```rust
config: &config::Config,
```
Notese que `config` se recibe **por referencia** (`&config::Config`), el constructor solo necesita *leer* esos valores para inicializar el hardware correctamente, no necesita tomar posesión (ownership) de la variable.

Una vez satisfechos estos parametros ya se puede inicializar el canal de comunicacion y el resultado se guarda en una variable mutable.

### Impresion
Para imprimir los mensajes UART se emplearon 2 macros principales y un metodo, cada una independiente de las otras. Se utilizaron para expresar los diferentes tipos de impresion en UART.
- `write!` & `writeln!` = Macros que se expanden en tiempo de compilacion, analizan el string con {} y generan el codigo que sustituye esos placeholders con el valor de la variable que este en ellos, convirtiendolo a texto automaticamente.
- `write` = Metodo normal, no tiene una logica para reemplazar variables dentro de {}, solo se encarga de exponer el texto que hay dentro.

Es importante destacar que write si podria imprimir variables dentro de si mismo, no obstante el metodo cambiaria y tendrias que hacer algo com oesto:
```rust
let texto = x.to_string(); // Primero convertir el número a String
uart.write(texto.as_bytes()).unwrap(); // Despues convertir el String a &[u8]
```

## Siguientes pasos
El contenido actual solo cubre el tema de escritura / enviar informacion mediante UART, falta por implementar:
- **Uso de CTS y RTS.**
- **Uso de funciones de lectura y recibir informacion.**  

Ambos temas seran cubiertos una vez se adquiera el hardware apropiado para su trabajo.  
Por otra parte por ahora no se incluyeron esquematicos en KiCAD ya que no se hizo uso de conexiones en protoboard con el microcontrolador para la etapa de escritura.