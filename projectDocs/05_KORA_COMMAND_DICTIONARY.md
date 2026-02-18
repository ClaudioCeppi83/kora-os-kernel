# 05. Especificación de Sintaxis y Diccionario de Comandos

## 1. Estructura Estándar de Comandos

Para garantizar la precisión en las órdenes dadas a los empleados digitales y al Kernel, todos los comandos siguen un patrón jerárquico estricto: `kora [grupo] [acción] [parámetros] --[banderas]`.

## 2. Grupos de Comandos Principales

### A. System (Gestión del Kernel y Operaciones)

Comandos para supervisar la salud del motor de IA y el túnel de comunicación nativa:

* **kora system status**: Muestra el estado del Bridge, la salud del proceso de OpenClaw y el consumo de recursos en tiempo real.
* **kora system restart**: Reinicia el túnel de comunicación nativa y refresca el contexto de la memoria del bot.
* **kora system logs**: Proporciona acceso a la auditoría de comandos y registros de error del sistema (Black Box).

### B. Agency (Gestión Multi-Tenancy)

Comandos destinados a administrar las diferentes unidades de negocio y sus agentes específicos:

* **kora agency list**: Enumera todas las agencias digitales configuradas en el sistema.
* **kora agency switch [nombre_agencia]**: Cambia el contexto operativo del Kernel a una agencia específica, activando su correspondiente `/knowledge` y `/workspace`.
* **kora agency create [nombre]**: Inicializa una nueva estructura simétrica de directorios para un nuevo negocio.

### C. Knowledge (RAG & Aprendizaje de Negocio)

Comandos para alimentar la base de conocimientos de la agencia con datos específicos:

* **kora knowledge learn [url/ruta]**: Activa el flujo de extracción (ej. YouTube, PDF, Docs), segmentación en bloques de 50KB y guardado automático en `/knowledge`.
* **kora knowledge query "[pregunta]"**: Realiza una consulta semántica rápida a la base de conocimientos sin iniciar una sesión de chat completa.
* **kora knowledge organize**: Ejecuta el protocolo de "Bibliotecaria" para clasificar archivos huérfanos dentro de la jerarquía de conocimientos.

### D. Workspace (Producción y Ejecución)

Comandos enfocados en la generación de activos técnicos y auditoría de proyectos:

* **kora workspace build [idea/componente]**: Ordena a OpenClaw generar archivos de código, diseños de funnels o activos técnicos directamente en `/workspace`.
* **kora workspace audit [ruta]**: Analiza el contenido de un proyecto y genera un reporte de calidad o diagnóstico.
* **kora workspace export [formato]**: Empaqueta los resultados del Workspace para su entrega final al cliente o plataforma externa.

## 3. Banderas Globales (Control de Ejecución)

Modificadores que alteran el comportamiento del Kernel durante la tarea:

* **--verbose (-v)**: Muestra el flujo técnico detallado y las llamadas internas entre KORA e OpenClaw (modo depuración).
* **--silent (-s)**: Ejecuta la tarea en segundo plano y utiliza el sistema de notificaciones para avisar al terminar.
* **--force (-f)**: Ignora advertencias de seguridad menores para permitir ejecuciones rápidas bajo responsabilidad del usuario.

## 4. Macros y Comandos Compuestos

KORA permite definir "Comandos Compuestos" en el archivo de configuración global, permitiendo automatizar flujos recurrentes (ej. Onboarding de nuevo cliente) con un solo alias.
