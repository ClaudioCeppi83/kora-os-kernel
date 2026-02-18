# 03. Modelo de Datos y Especificación de Persistencia Local

## 1. Arquitectura Híbrida de Almacenamiento
KORA no depende de un único motor; utiliza tres capas de almacenamiento para garantizar velocidad, legibilidad humana y seguridad inmutable:

| Capa | Tecnología | Función en KORA |
| :--- | :--- | :--- |
| **Relacional** | SQLite (`kora_kernel.db`) | Índice de conocimientos (RAG), metadatos de archivos y configuración de sesiones. |
| **Auditoría** | JSONL (Append-only) | Historial de comandos y logs de seguridad inmutables y resistentes a la corrupción. |
| **Blobs** | Filesystem (`/knowledge` & `/workspace`) | Documentos crudos, activos generados y transcripciones masivas. |

## 2. Esquema de Base de Datos (Tablas Nucleares)
El archivo `data/kora_kernel.db` contiene la inteligencia estructural del sistema para todas las agencias gestionadas:
*   **knowledge_index**: Almacena ID, nombre de archivo, etiquetas, fecha de indexación y la ruta al resumen generado por OpenClaw.
*   **sessions**: Gestiona el estado de los empleados digitales: `session_id`, `agent_id`, tiempo de inicio y tokens de contexto utilizados.
*   **plugin_registry**: Controla los permisos de los drivers (nombre, versión, nivel de permisos y estado).

## 3. El "Session Vault" (Bóveda de Sesión)
Para que OpenClaw no pierda el progreso de sus tareas ante fallos críticos o reinicios del proceso, KORA implementa un sistema de snapshots:
*   **Captura de Estado**: Al final de cada interacción crítica, KORA vuelca un "Snapshot de Contexto" al host.
*   **Reinyección Automática**: Si el entorno de ejecución se recrea, el Kernel reinyecta los estados críticos guardados para que el usuario no perciba interrupciones en la tarea técnica.

## 4. Shadow Metadata (Privacidad PII)
Para proteger datos sensibles de los clientes (nombres, emails, teléfonos), KORA implementa una capa de anonimización:
*   **Índice de Metadatos Sombra**: Se genera en la base de datos local.
*   **Aislamiento del LLM**: OpenClaw solo interactúa con resúmenes técnicos o IDs anonimizados, mientras que los datos reales permanecen cifrados en el sistema de archivos del host.

## 5. Integridad y Auditoría Inmutable
Cada acción realizada por el Agente o el Kernel se registra en `logs/kora_audit.log` utilizando un encadenamiento de integridad:
*   **Estructura Atómica**: Las entradas incluyen TIMESTAMP, ACTOR (USER/KORA/AGENT), ACTION, RESULT y un HASH de integridad.
*   **Encadenamiento SHA-256**: Cada entrada genera un hash basado en el contenido actual y el hash de la entrada anterior, asegurando que los registros no puedan ser alterados maliciosamente.
*   **Checkpoints**: KORA genera copias de seguridad automáticas de la base de datos cada vez que se completa un hito en el Workspace.
