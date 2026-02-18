# 07. Protocolo de Seguridad y Auditoría de Grado Empresarial

## 1. Filosofía de Confianza Cero (Zero Trust)
En KORA OS, la seguridad no es un perímetro, sino una condición intrínseca de cada proceso. El sistema asume que el entorno de ejecución del agente (Ring 3) es potencialmente inestable y, por lo tanto, toda interacción con el mundo exterior debe ser mediada, filtrada y registrada por el Kernel (Ring 1).

## 2. La "Black Box" (Auditoría Inmutable)
Cada decisión, comando y resultado generado por OpenClaw se registra en un log de auditoría JSONL diseñado para ser una prueba forense del comportamiento del sistema.

*   **Encadenamiento Criptográfico**: Para prevenir la alteración de registros, cada entrada $H_n$ se vincula a la anterior mediante el protocolo:
    $$H_n = \text{SHA256}(Event_n + H_{n-1})$$
*   **Persistencia Dual**: Los logs se escriben simultáneamente en el almacenamiento local y, si hay conexión, se replican en el Hardware Pro para evitar la pérdida de trazabilidad en dispositivos Lite.
*   **Alertas de Integridad**: Si el Kernel detecta una ruptura en la cadena de hashes durante la sincronización, el sistema bloquea automáticamente la agencia afectada.
*   **Interrupción Síncrona (Bridge Lock)**: El Native Bridge bloquea físicamente la escritura en el PTY cuando se activa un Security Gate (Ring 0), impidiendo que el agente inyecte comandos adicionales hasta recibir la firma criptográfica del usuario.

## 3. Gestión de Secretos y Hardware Root of Trust
KORA protege las llaves de tu negocio (API Keys, credenciales, accesos CRM) mediante capas de abstracción que impiden que el agente vea los datos sensibles:

*   **Hardware Pro (TPM 2.0)**: Las llaves maestras se almacenan en el módulo de plataforma segura del hardware, fuera del alcance de cualquier software.
*   **Secret Vault (Capa de Software)**: Los plugins solicitan "autorización de uso" al Kernel. El Kernel inyecta la credencial en la cabecera de la petición de forma transparente; OpenClaw solo maneja el resultado de la operación, nunca la llave.
*   **Scrubbing de Salida**: El Kernel escanea las respuestas del agente antes de mostrarlas en la UI para detectar y censurar posibles fugas de información sensible (PII).

## 4. Sandboxing y Aislamiento de Ejecución
OpenClaw opera dentro de un entorno restringido por el Kernel de Linux (Namespaces y Cgroups) con las siguientes limitaciones:

*   **Network Isolation**: El agente no tiene acceso a internet por defecto. Todas las peticiones web se realizan a través del Proxy de KORA, que valida las URLs contra una "Lista Blanca" de la agencia.
*   **Filesystem Jail**: El proceso del agente solo puede ver los directorios `/knowledge` y `/workspace`. Cualquier intento de acceso a rutas del sistema host dispara un Hard Reset preventivo.

## 5. Protocolo de Resiliencia y Recuperación (Auto-Healing)
KORA supervisa la salud del agente continuamente para garantizar la continuidad operativa:

*   **Soft Recovery**: Reinicio del hilo de comunicación si se detecta un bloqueo de STDOUT superior a 30 segundos.
*   **Hard Reset**: Si el agente intenta violar un anillo de seguridad (Ring 1 o 0), el Kernel termina el proceso de OpenClaw, limpia el Session Vault corrupto y restaura el sistema al último "Punto de Verdad" conocido.

## 6. Validación de Integridad Sync-Stream
Para el flujo entre el dispositivo Lite y el Pro, KORA ejecuta un "Check-in" de seguridad:
1.  **Hash Verification**: Se recalcula toda la cadena de auditoría del dispositivo Lite.
2.  **Behavioral Audit**: Se analizan los logs en busca de patrones de comandos no autorizados durante el modo offline.
3.  **Atomic Fusion**: Solo si la validación es 100% exitosa, se integran los cambios en la base de datos maestra del Appliance.
