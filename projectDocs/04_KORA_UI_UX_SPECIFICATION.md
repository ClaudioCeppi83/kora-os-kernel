# 04. Manifiesto Maestro de Interfaz y Gobernanza Visual

## 1. Filosofía y Concepto: Cyber-Industrial Bento

KORA OS implementa una estética **"Cyber-Industrial"** de alta fidelidad diseñada para maximizar la visibilidad operativa y la soberanía del hardware. La interfaz se organiza bajo el concepto de **"Bento Box"**: una estructura modular de contenedores con bordes definidos y efectos de iluminación rítmicos que permiten una supervisión técnica profunda sin saturación cognitiva.

## 2. Identidad de Marca y Glifos del Kernel

La marca no es un elemento cosmético, sino una señal de integridad del sistema.

* **El Glifo Maestro ("The Data Monolith")**: Representa la integridad del Kernel y el procesamiento de datos por bloques de 50 KB.
  * **Construcción**: Líneas horizontales paralelas que emulan láminas de hardware.
  * **Dinamismo**: Inclinación hacia la derecha (velocidad de ejecución) con un rastro binario que simboliza el flujo del **Native Bridge**.
* **Firma del Kernel (Top Bar Wordmark)**: Texto "kora os" construido con láminas horizontales en **Oro KORA (#D4B235)**.
  * **Dimensiones**: Altura de 18px, alineado a la izquierda con un margen de 24px antes del menú "FILE".
  * **Comportamiento**: Emite un pulso de brillo (**Glow**) sincronizado con la actividad del Anillo 0.

## 3. Estándares Técnicos y Estilo Visual

### A. Paleta Semántica y Contraste

Esquema diseñado para reducir la fatiga en entornos de baja luz y resaltar acciones críticas:

* **Fondo Primario (Obsidian Black)**: `#121212` (Lienzo base).
* **Fondo Secundario (Neutral Dark)**: `#1A1A1A` (Módulos Bento y fondos de menús).
* **Señal de Sistema (Oro KORA)**: `#D4B235` (Elements activos, cursor y bordes de enfoque).
* **Acción Crítica (Rust Red)**: `#C23B22` (Cabeceras de impacto, errores y estados de alerta).
* **Texto Neutral**: `#E6E6E6` (Lectura general técnica).

### B. Tipografía y Grilla

* **Display (Space Grotesk)**: Títulos de módulos y etiquetas de menús.
* **Terminal (JetBrains Mono)**: Kernel Shell, logs de auditoría SHA-256 e metadatos.
* **Especificaciones Bento Grid**:
  * **Gutter**: Margen uniforme de **12px** entre módulos.
  * **Borders**: Grosor de 1px con opacidad reducida (`primary/20`).
  * **Radius**: Esquinas vivas (**0px**) de carácter industrial.

### C. Capas FX

* **Efecto Scanline**: Capa persistente `rgba(212, 178, 53, 0.03)` que recorre el viewport para simular un monitor industrial.

## 4. Arquitectura de Navegación y Menús

### A. Menú Global (Top Bar)

Acceso persistente a las funciones raíz del sistema:

* **File**: Gestión de sesiones y agencias.
* **Tools**: Drivers externos y utilidades.
* **Knowledge**: Administración de biblioteca RAG.
* **System**: Configuración de hardware Pro/Lite.
* **Interacción**: Despliegue vertical con Hotkeys alineadas a la derecha en Oro KORA.

### B. Menú Contextual (File Operations)

Activado por Ring 0 sobre el explorador de archivos:

* **Diseño**: Cabecera en sólido **Rust Red (#C23B22)** y cuerpo en **Neutral Dark (#1A1A1A)**.
* **Funciones**: Análisis con OpenClaw, transferencia al Workspace y visor de auditoría.

## 5. Distribución de Paneles Funcionales

* **System Monitor**: Gráficos SVG en tiempo real para CPU y RAM con gradientes dinámicos.
* **Agent Status**: Widget de estado para OpenClaw con métricas de latencia neuronal.
* **File Explorer**: Árbol de directorios con distinción cromática (carpetas `/logs` en Rust Red) e iconos de sistema maquinados.
* **Kernel Shell**: Instancia de **Xterm.js** conectada a un **PTY real** (Rust-Backend) con aceleración WebGL.

## 6. Seguridad y Supervisión (Security Gates)

Cualquier acción crítica que afecte al Ring 0 activa una **Interrupción Síncrona**:

* **Bloqueo de PTY**: El Native Bridge congela el flujo de datos.
* **Modal de Aprobación**: Se requiere firma explícita del usuario para liberar la acción hacia los drivers del Ring 2.
* **Auditoría**: Cada evento se registra mediante encadenamiento SHA-256 inmutable.
