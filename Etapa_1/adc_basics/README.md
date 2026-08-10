## Guia de funcionamiento
### Descripcion
Esta practica consistio en la implementacion basica de ADC via `oneshot` en el microcontrolador ESP32-S3 aprovechando el uso de un multimetro para apreciar el cambio constante del valor obtenido al mover la perilla. Cabe resaltar que no se implemento un modo `continuous` para ADC debido a que encajaba con la filosofia de introduccion de la practica.   

### Herramientas
- 1 Potenciometro (Se utilizo uno de 1k)

### Codigo
Esta practica consistio en la aplicacion y estudio del `ADC` (Analog to Digital Converter) de una ESP32 el cual en terminos simples busca convertir un voltaje en un punto a un numero. Para realizarlo se apoyo de un potenciometro cuyos pines extremos iban conectados a los pines de 3.3V y GND de la ESP32, mientras que el pin central iba conectado directamente al pin encargado de interpretar el voltaje. El uso del potenciometro fue clave debido a que al girar la perilla en un sentido, el voltaje en el cursor se acerca a 3.3V (leyendo valores más altos); al girar en el sentido opuesto, se acerca a 0V (valores más bajos).

#### Modulos nuevos importados
```rust
use esp_idf_svc::hal::adc::oneshot::{...}; // Modulo encargado de exportar funciones dedicadas al control de ADC via oneshote
use esp_idf_svc::hal::adc::attenuation::DB_12; // Modulo encargado de centrar la atenuacion de ADC a 12 decibeles
```
Ambos modulos utilizados provienen de la capa de abstraccion `HAL` y traen sus propias funciones y registros (structs) que se veran a continuacion:

#### Asignacion de un pin adc dedicado
Para comenzar a operar con ADC, el codigo necesita utilizar un pin interno especifico, el cual que no es el que controlara directamente el GPIO de la placa, sino que se encargara de la lectura interna del adc en el microcontrolador. Para ello creamos una variable que contendra un registro (struct):
```rust
let adc = AdcDriver::new(...).unwrap();
```
Notese que se esta asignando el struct `AdcDriver` el cual internamente luce asi:
```rust
pub struct AdcDriver<'d, U> {
        handle: adc_oneshot_unit_handle_t,
        _unit: PhantomData<U>,
        _t: PhantomData<&'d mut ()>,
    }
```
Del mismo modo se le asigna la funcion new la cual espera recibir los siguientes parametros:   
```rust
pub fn new<ADC: Adc<AdcUnit = U> + 'd>(_adc: ADC) -> Result<Self, EspError> {
        let config = adc_oneshot_unit_init_cfg_t {
            unit_id: ADC::unit(),
            ..Default::default()
        };
```   
Notese que espera recibir un pin interno del microcontrolador --el cual no es un gpio convencional-- para operar, no obstante, la ESP32 opera con dos pines especificos para trabajar con adc:
- `peripherals.adc1`
- `peripherals.adc2`

Se recomienda trabajar con el pin adc1 en esta clase de practicas o trabajos debido a que el pin adc2 suele presentar conflictos cuando el WiFi del microcontrolador está activo, ya que ambos comparten el mismo hardware interno, por eso se recomienda usar adc1 salvo que no se vaya a usar WiFi en el proyecto
Entonces la linea de este codigo queda asi
```rust
let adc = AdcDriver::new(peripherals.adc1).unwrap();
```
Creando un registro (struct) de configuracion al cual se le asigna internamente el pin adc1 para trabajar en esta practica y el resultado -- OK || ERROR -- lo maneja el `.unwrap`.  

#### Bloque de configuracion del ADC
Como es costumbre en estas practicas, antes de implementar el protocolo ADC, el microcontrolador espera seguir una configuracion especifica para operar, es por ello que se crea una variable que almacenara la configuracion del registro (struct) `AdcChannelConfig`:
```rust
let adc_config = AdcChannelConfig{...};
```
Siendo que internamente `AdcChannelConfig` espera recibir los siguientes parametros:
```rust
pub struct AdcChannelConfig {
    pub attenuation: adc_atten_t, // Determina el rango máximo de voltaje que el pin puede medir de forma segura
    pub resolution: Resolution, // Especifica el número de bits utilizados para cuantificar la señal
    pub calibration: Calibration, // Permite compensar las variaciones de fabricación del Chip (Vref) o ajustar una curva de corrección personalizada para convertir valores digitales a milivoltios
}
```
Para esta practica el unico parametro que se modifico fue el parametro de `attenuation`, asignandole 12 Decibeles como limite seguro. El ersto de parametros se trabajaron de manera default.
```rust
let adc_config = AdcChannelConfig{ 
    attenuation: DB_12,
    ..Default::default()
    }; 
```

#### Asignacion en formato Output para el ADC
Con la configuracion para el canal de ADC y el pin interno encargado de operar como ADC, queda exportar esta informacion a un pin GPIO con estos parametros, por ello se utiliza la funcion `new` asignada al registro (struct) `AdcChannelDriver`:
```rust
let mut adc_pin = AdcChannelDriver::new(...).unwrap(); 
```
Internamente este struct de canal para el driver del ADC luce asi:
```rust
pub struct AdcChannelDriver<'d, C, M>
    where
        C: AdcChannel,
        M: Borrow<AdcDriver<'d, C::AdcUnit>>,
    {
        adc: M,
        _channel: PhantomData<C>,
        converter: Converter,
        _t: PhantomData<&'d mut ()>,
    }
```
Mientras que su funcion `new` realiza lo siguiente
```rust
pub fn new(
    adc: M,
    pin: impl ADCPin<AdcChannel = C> + 'd,
    config: &AdcChannelConfig,
) -> Result<Self, EspError>
```
Notese que espera recibir el pin asignado al ADC, el pin asignado de GPIO y la configuracion del canal que ya realizamos previamente  
Es por ello que en el codigo se trabajo de la siguiente manera
```rust
let mut adc_pin = AdcChannelDriver::new( 
        &adc, 
        pins.gpio7,
        &adc_config).unwrap(); 
```
Siendo:
- `adc` La variable que guarda al pin asignado internamente.
- `pins.gpio7` Se utilizara el pin numero 7
- `adc_config` El bloque de configuracion del canal de ADC

### Resultados obtenidos  
I (211776) adc_basics: ADC value: 0  
I (211876) adc_basics: ADC value: 0  
I (211976) adc_basics: ADC value: 0  
I (212076) adc_basics: ADC value: 215  
I (212176) adc_basics: ADC value: 426  
I (212276) adc_basics: ADC value: 641  
I (212376) adc_basics: ADC value: 841  
I (212476) adc_basics: ADC value: 1024  
I (212576) adc_basics: ADC value: 1281  
I (212676) adc_basics: ADC value: 1557  
I (212776) adc_basics: ADC value: 1797  
I (212876) adc_basics: ADC value: 1860  
I (212976) adc_basics: ADC value: 1860  
I (213076) adc_basics: ADC value: 1876  
I (213176) adc_basics: ADC value: 2216  
I (213276) adc_basics: ADC value: 2746  
I (213376) adc_basics: ADC value: 2928  
I (213476) adc_basics: ADC value: 2925  
I (213576) adc_basics: ADC value: 2963  
I (213676) adc_basics: ADC value: 3100  
I (213776) adc_basics: ADC value: 3100  
I (213876) adc_basics: ADC value: 3100  
I (213976) adc_basics: ADC value: 3100  
I (214076) adc_basics: ADC value: 3100  