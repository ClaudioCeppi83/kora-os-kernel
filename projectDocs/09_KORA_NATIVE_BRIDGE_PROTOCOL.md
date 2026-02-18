# 09. Protocolo del Native Bridge y Comunicación Híbrida

## 1. Definición del Túnel de Ejecución
El **Native Bridge** actúa como el sistema nervioso central de KORA OS, conectando la interfaz de alta fidelidad (Frontend) con el núcleo de ejecución en Rust (Backend). A diferencia de los web-wrappers convencionales, este puente utiliza un bus binario de baja latencia para garantizar que el **Kernel Shell** opere como un terminal real y no como una simulación.

## 2. Estándar de Comunicación: KORA-RPC
Para asegurar la integridad y la velocidad, el sistema implementa un protocolo basado en **JSON-RPC sobre IPC (Inter-Process Communication)**:
*   **Serialización**: Los comandos y respuestas se empaquetan en formato binario para minimizar el overhead de procesamiento en el Ring 1.
*   **Asincronía**: El bridge maneja múltiples hilos de ejecución, permitiendo que el **System Monitor** se actualice en tiempo real sin bloquear el flujo de datos del terminal.

## 3. Gestión del Terminal PTY Real
El Kernel Shell integra **Xterm.js** conectado a un proceso **PTY (Pseudo-Terminal)** nativo gestionado por Rust:
*   **Flujo Bi-direccional**: El bridge canaliza `STDIN` (entrada del usuario) y `STDOUT/STDERR` (salida del agente/sistema) con latencia cercana a cero.
*   **Aislamiento de Señales**: El bridge captura señales de interrupción (como `Ctrl+C`) y las traduce en comandos de control para el proceso de OpenClaw en el Ring 3, evitando cierres inesperados del Kernel.

## 4. Implementación de Security Gates
El protocolo del bridge incluye un estado de **"Interrupción de Seguridad"**:
1.  **Detección**: Cuando OpenClaw propone una acción que requiere privilegios de Ring 0, el Kernel envía una señal de bloqueo al bridge.
2.  **Congelación de UI**: El bridge suspende temporalmente el flujo del PTY y dispara el modal de aprobación en la interfaz.
3.  **Validación de Firma**: El comando solo se libera hacia los drivers del Ring 2 tras recibir la firma digital del usuario desde la interfaz.

## 5. Resiliencia del Enlace
*   **Auto-Reconnect**: Si el hilo de la interfaz pierde sincronía con el proceso PTY, el bridge realiza un "Hot Reload" del estado visual sin perder el snapshot de contexto actual.
*   **Heartbeat**: Un pulso de 100ms verifica la salud del túnel; si falla, el Kernel dispara un **Soft Recovery** preventivo.
