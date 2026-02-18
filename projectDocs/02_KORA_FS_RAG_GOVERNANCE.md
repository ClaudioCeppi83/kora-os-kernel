# 02. Gobernanza del Sistema de Archivos y RAG

## 1. Estructura Física y Simetría

Para garantizar la portabilidad entre el Hardware Pro y el dispositivo Lite, KORA exige una jerarquía de archivos estricta donde todas las rutas internas deben ser relativas a la raíz del proyecto. El Kernel (Ring 1) intercepta y valida cada petición de archivo realizada por OpenClaw.

### Directorios Principales

* **/src**: Código fuente del Kernel y el Native Bridge (Python 3.10+ / Rust).
* **/knowledge**: Biblioteca RAG organizada para segmentar el conocimiento masivo del negocio.
* **/workspace**: Área segura de salida donde OpenClaw deposita los activos generados para validación del usuario (Ring 0).
* **/plugins**: Módulos de extensión y drivers de servicio con sus propios manifiestos.
* **/data**: Persistencia local (SQLite y Snapshots de contexto).

## 2. El Árbol del Conocimiento (Knowledge Tree)

KORA organiza automáticamente la carpeta `/knowledge` para evitar la fuga de contexto entre diferentes procesos o agencias:

* **/knowledge/core**: Reglas maestras, identidades de agentes y principios de "Verdad Absoluta".
* **/knowledge/archive**: Datos crudos procesados y transcripciones masivas (ej. extracciones de documentos o media).
* **/knowledge/dist**: Análisis estructurados y técnicos listos para ser consumidos por el motor de ejecución.

## 3. Motor de Indexación y Fragmentación (RAG)

KORA actúa como una MMU (Memory Management Unit) para OpenClaw, garantizando latencia cero en la fragmentación de activos mediante técnicas de **"Zero-Copy"** en la capa de Rust. Evita el desbordamiento de contexto mediante:

### Protocolo de Fragmentación (Contextual Chunking)

* **Tamaño de Bloque**: Segmentación automática de archivos en páginas de **50 KB**.
* **Traslape Semántico**: Se mantiene un **10%** de traslape entre fragmentos para preservar la continuidad del significado.
* **Detección Automática**: El Watch-Driver utiliza eventos del kernel (Inotify) para detectar nuevos archivos y emitir una señal de `INDEX_REQUEST`.

## 4. Aislamiento Multi-Tenancy

Para asegurar que los datos de un cliente o agencia no se filtren a otros, el Kernel impone:

* **Rutas Relativas**: OpenClaw no puede invocar rutas absolutas del sistema operativo host.
* **Validación de Escritura**: Las salidas solo son válidas si se escriben en subcarpetas de `/workspace` previamente autorizadas.
* **Segmentación por Agencia**: Cada unidad de negocio digital debe residir en una jerarquía de carpetas raíz independiente, gestionada por el Kernel.

## 5. Protocolo de Análisis por Etapas

Para archivos masivos (> 200 KB), KORA obliga a OpenClaw a seguir un flujo estructurado:

1. **Indexación**: Mapeo de encabezados y resúmenes de cada bloque de 50 KB.
2. **Focalización**: Procesamiento de secciones específicas identificadas por el usuario o el plan de trabajo.
3. **Síntesis**: Reconstrucción global basada en los resultados parciales almacenados en `/knowledge/dist`.
