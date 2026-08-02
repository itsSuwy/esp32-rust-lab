# Rust_ESP32_STD
## Descripcion
Este repositorio nacio de la idea de tener un laboratorio personal de proyectos utilizando una placa de desarrollo ESP32, no obstante, a diferencia de repositorios o proyectos utilizando el mismo hardware, aqui se desarrollaran proyectos con el lenguaje de programacion "**Rust**", mas especificamente a su version STD en arquitectura Xtensa respaldada por Espressif. Ya que Rust en sistemas embebidos, microcontroladores y microprocesadores esta ganando fuerza, por lo que me gustaria aprender y dominar este ambito (no me cierro a trabajar con la version no std a futuro!).  
Por otra parte me gustaria que este repositorio pudiera servir de ayuda con los proyectos ya realizados a quienes estan empezando en este ecosistema, ya que encontrar informacion orientada a std, almenos para mi se me complico encontrar.

## Objetivos del repositorio
Ademas de documentar proyectos realizados y poder ayudar a personas nuevas, me gustaria que el destino manifiesto de este proyecto se apegara a los siguientes lineamientos
- Aprender Rust en sistemas **std**.
- Documentar cada proyecto desde cero.
- Presentar ejemplos reproducibles.
- Mantener buenas practicas en el desarrollo.
- Priorizar soluciones técnicamente correctas y eficientes, manteniendo un código claro y mantenible.
- Aspirar a trabajar con proyectos mas "demandantes" segun vaya progresando en los trabajos.

## Contenido
Dentro de este repositorio existiran multiples carpetas asociadas a un proyecto especifico (blinking-leds, comunicacion UART, etc) las cuales contendran toda la informacion que se utilizo para realizarlo. Dentro de cada trabajo la informacion mantendra un orden jerarquizado en dos categorias principales:
- Codigo: Toda la logica para el funcionamiento, incluira un README.md que explique hace cada parte del codigo asi como los componentes que se utilizaron para crearlo con una protoboard. Segun se progrese en la complejidad del trabajo, se comenzaran a omitir ciertos elementos que ya se explicaron en proyectos pasados, para asi evitar saturar el README.md de informacion que ya ha aparecido en otros proyectos, siguiendo un estilo KISS.
- Hardware: Una carpeta exclusiva dentro del directorio del proyecto, contendra: archivo esquematico, diseño de PCB, Project File, los tres siendo nativos de KiCAD. Tambien se adjuntara una o varias fotos del circuito armado en una Protoboard funcionando.

## Cronologia de actividades
Mi meta es pasar de lo basico a lo particular/nicho con este trabajo orientandolo mas a sistemas IoT, por ende decidi separar por etapas los proyectos a realizar, cada una conteniendo varios proyectos y respetando su enfoque comun. Las fases que decidi seguir son:

### Etapa 1: Fundamentos de hardware abstraction
Primera etapa del aprendizaje. Aqui se desarrollaran las bases para el trabajo en sistemas embebidos enfocadas a comprender el funcionamiento del `HAL` (Hardware Abstraction Layer) proporcionado por la libreria oficial de Espressif para abstraer el acceso al ESP-IDF desde Rust. Se aprende a interactual con los perifericos fundamentales del microcontrolador aplicando funciones de: GPIO, UART, PWM, ADC y temporizadores. Estableciendo los conceptos que serviran de base para etapas posteriores.

### Etapa 2: Patrones de Diseño para Sistemas Embebidos
Etapa enfocada en la construcción de aplicaciones embebidas mediante patrones de diseño fundamentales. A partir de los periféricos estudiados anteriormente, se implementan máquinas de estados, integración de múltiples módulos, control de dispositivos de salida y procesamiento básico de datos provenientes de sensores. El objetivo es desarrollar un diseno de codigo modular, escalable y fácil de mantener, siguiendo una arquitectura que pueda reutilizarse en proyectos de mayor complejidad.

### Etapa 3: Comunicación entre Plataformas
Etapa enfocada en el diseño e implementación de arquitecturas distribuidas mediante la comunicación entre múltiples microcontroladores. Se desarrollan protocolos de intercambio de datos, análisis e interpretación de comandos, asignación de responsabilidades entre dispositivos y coordinación del comportamiento del sistema mediante máquinas de estados distribuidas. El objetivo es comprender cómo construir sistemas embebidos compuestos por varios nodos que colaboran entre sí de forma confiable y escalable.

### Etapa 4: Buses de Comunicación
Etapa dedicada al aprendizaje de los principales protocolos de comunicación utilizados en sistemas embebidos para la interacción con periféricos externos. Se implementan interfaces basadas en **I2C** y **SPI**, comprendiendo su funcionamiento, la conexión de múltiples dispositivos sobre un mismo bus y el desarrollo de interfaces gráficas básicas para la visualización de información. El objetivo es adquirir las bases necesarias para integrar sensores, memorias, pantallas y otros periféricos en aplicaciones embebidas de mayor complejidad.

### Etapa 5: Integración de Sensores
Etapa orientada a la adquisición y procesamiento de datos provenientes de sensores utilizados en aplicaciones embebidas reales. Se trabaja con sensores ambientales, de movimiento, de distancia y convertidores analógico-digitales de alta resolución, abordando tanto protocolos de comunicación como requisitos específicos de temporización y precisión. El objetivo es desarrollar la capacidad de integrar múltiples fuentes de información para construir sistemas capaces de percibir e interpretar su entorno.

### Etapa 6: Comunicación Inalámbrica
Etapa dedicada al desarrollo de sistemas embebidos capaces de intercambiar información sin conexión física. Se implementan enlaces mediante transceptores de radiofrecuencia y redes Wi-Fi, abordando el diseño de protocolos de comunicación, el intercambio confiable de datos y la integración con servicios IoT utilizando MQTT. El objetivo es comprender las bases de la conectividad inalámbrica aplicada a sistemas embebidos modernos.

### Etapa 7: Redes LoRa
Etapa enfocada en el desarrollo de sistemas de comunicación de largo alcance y bajo consumo utilizando tecnología LoRa. Se implementan enlaces punto a punto, nodos sensores autónomos y arquitecturas de adquisición de datos con múltiples dispositivos, comprendiendo los principios de las redes de telemetría para aplicaciones distribuidas e IoT.

### Etapa 8: Proyectos Integradores
Etapa destinada a consolidar los conocimientos adquiridos mediante el desarrollo de aplicaciones embebidas completas. Cada proyecto integra múltiples periféricos, protocolos de comunicación, sensores, actuadores y patrones de diseño estudiados en las etapas anteriores, permitiendo aplicar los conceptos en escenarios representativos de sistemas embebidos reales antes de avanzar hacia contenidos de mayor complejidad.
