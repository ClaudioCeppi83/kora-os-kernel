# 04. Especificación de Interfaz UI/UX: The Bento Box

## 1. Filosofía Visual: Cyber-Industrial Bento
KORA OS utiliza una estética "Cyber-Industrial" de alta fidelidad para maximizar la visibilidad operativa. La interfaz está diseñada como una **"Bento Box"** modular que organiza la información en contenedores con bordes definidos y efectos de iluminación sutiles, permitiendo una supervisión técnica profunda sin saturación cognitiva.

## 2. Atributos Estéticos y Estilo
La interfaz implementa efectos visuales específicos para reforzar la identidad de "Kernel de Inteligencia":
*   **Efecto Scanline**: Una capa de líneas de escaneo sutiles (`rgba(212, 178, 53, 0.03)`) recorre la pantalla para simular un monitor industrial.
*   **Tipografía**:
    *   **Display**: *Space Grotesk* para títulos, menús y elementos de interfaz.
    *   **Terminal**: *JetBrains Mono* para el Kernel Shell y datos de sistema.
*   **Efectos de Brillo (Glow)**: Indicadores de estado con sombras difusas (ej. glow-green para agentes activos), renderizados vía CSS optimizado para no afectar el framerate.
*   **Bordes Bento**: Bordes finos de 1px con opacidad reducida (`primary/20`) para delimitar los módulos.

## 3. Paleta de Colores Corporativa
*   **Fondo Principal (Background)**: `#121212` (Dark Canvas).
*   **Color Primario (Oro KORA)**: `#D4B235` - Utilizado para acentos, bordes activos y el cursor de la terminal.
*   **Estado Alerta (Rust)**: `#C23B22` - Para logs de errores, carpetas críticas y estados de advertencia.
*   **Fondo Secundario (Neutral Dark)**: `#1A1A1A` - Para el fondo de los módulos Bento y menús.
*   **Texto Neutral**: `#E6E6E6` - Para la lectura general.

## 4. Arquitectura de Menús y Navegación

### A. Menú Global (Dropdown)
Ubicado en la Top Navigation Bar. Proporciona acceso persistente a las funciones raíz:
*   **File**: Gestión de sesiones, importación/exportación de agencias.
*   **Tools**: Acceso a drivers externos y utilidades de sistema.
*   **Knowledge**: Administración de la biblioteca RAG y re-indexación.
*   **System**: Configuración del hardware Pro y sincronización Lite.

### B. Menú Contextual (File Operations)
Activado mediante clic derecho en el File Explorer. Permite acciones rápidas sobre archivos y carpetas:
*   **Analizar con OpenClaw**: Envía el archivo al proceso de fragmentación contextual.
*   **Mover al Workspace**: Marca activos para validación de salida.
*   **Ver Auditoría**: Abre los logs específicos asociados a ese recurso.

## 5. Distribución de Paneles
*   **System Monitor**: Visualización de gráficos SVG en tiempo real para CPU y RAM, con gradientes de color según carga.
*   **Agent Status**: Widget dedicado al estado de OpenClaw con indicador de latencia del motor neuronal.
*   **File Explorer**: Árbol de directorios con iconos de sistema (Material Symbols) y distinción cromática para carpetas de sistema (ej. `/logs` en color Rust).
*   **Kernel Shell**: Instancia de **Xterm.js** conectada a un **PTY real** con aceleración **WebGL**. No es una simulación; ofrece emulación completa de terminal VT100/xterm-256color con latencia nativa.

## 6. Controles de Supervisión (Security Gates)
Cualquier acción propuesta por OpenClaw que requiera ejecución en el Ring 0 (ej. envío de correos, cambios en base de datos) activa una **Interrupción Síncrona**: el Native Bridge congela el flujo del PTY y bloquea el renderizado de la UI disparando un modal de aprobación. Ningún dato puede entrar o salir del shell hasta que el usuario firme la acción.
