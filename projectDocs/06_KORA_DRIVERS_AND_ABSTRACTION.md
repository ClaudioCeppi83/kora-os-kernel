# 06. Drivers de Sistema y Abstracción de Recursos

## 1. Definición de Drivers en KORA

Los drivers son módulos de abstracción que actúan como intermediarios entre el Kernel (Ring 1) y los recursos externos. Su función principal es evitar que OpenClaw (Ring 3) tenga acceso directo al hardware o a la red, encapsulando acciones lógicas en funciones seguras y estandarizadas.

## 2. Tipos de Drivers Nucleares (Core Drivers)

KORA implementa tres drivers esenciales para la operación de las agencias digitales:

* **Notify-Driver**: Actúa como puente con el sistema de avisos del sistema operativo host (ej. `libnotify`). Permite que el agente notifique al usuario sobre tareas finalizadas sin depender de la terminal abierta.
* **Watch-Driver**: Monitoriza en tiempo real el sistema de archivos local mediante eventos del kernel (`Inotify`). Detecta nuevos datos en `/knowledge` para activar automáticamente el flujo de aprendizaje RAG.
* **Exec-Host-Driver**: Ejecuta scripts específicos en el host que no pueden correr dentro del entorno restringido del agente. Implementa un sistema de "Capacidades" donde cada script debe estar pre-aprobado y firmado.

## 3. Arquitectura de Plugins y Extensiones

Para interactuar con servicios externos (CRMs, APIs, automatización), KORA utiliza un sistema modular de plugins:

### Estructura de un Plugin

Cada plugin reside en `/plugins/[nombre]` y debe contener:

* **manifest.json**: Metadatos, versión y nivel de permisos requeridos.
* **logic.py** o **handler.sh**: Código ejecutable que realiza la tarea técnica bajo supervisión del Kernel.
* **instructions.md**: Manual semántico que el Kernel inyecta en OpenClaw para que el bot comprenda cómo invocar el plugin.

## 4. Ciclo de Vida y Eventos (Hooks)

Los plugins pueden integrarse en el flujo operativo de KORA mediante puntos de control específicos:

* **BEFORE_COMMAND**: Valida o sanitiza los datos de entrada antes de llegar al ejecutor.
* **AFTER_SUCCESS**: Ejecuta rutinas de limpieza o actualiza el índice de conocimientos tras una tarea exitosa.
* **ON_ERROR**: Define protocolos de recuperación o reportes de fallo técnico.

## 5. Protocolo de "Petición de Capacidad"

KORA oculta la complejidad del hardware al agente mediante una capa de decision lógica:

1. **Petición**: OpenClaw no solicita acceso a un recurso (ej. un puerto), solicita una acción lógica (ej. "Validar conexión externa").
2. **Mediación**: El Kernel decide qué driver utilizar y valida si la agencia tiene los permisos necesarios.
3. **Aislamiento de Credenciales**: El agente nunca ve los tokens de autenticación; solo envía parámetros de datos al driver, que gestiona las llaves de forma aislada.
