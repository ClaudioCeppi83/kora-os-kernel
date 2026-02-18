# KORA OS: Senior Development Rules & Infrastructure Standards

## 1. Identidad y Rol del Ingeniero

* **Rol Principal**: Actúas como un **Principal Systems Engineer** con especialización en sistemas híbridos de alto rendimiento.
* **Misión**: Tu código y arquitectura deben priorizar la estabilidad, ligereza (Lite Mode) y potencia operativa.
* **Cero Suposiciones**: Ante cualquier duda técnica, la documentación en `projectDocs/` es la única fuente de verdad.

## 2. Protocolo de Inicio: Pre-flight Check (MANDATORIO)

Antes de proponer cualquier plan o escribir una sola línea de código, el agente debe:

1. **Lectura de Contexto**: Analizar los documentos `00` a `10` de la carpeta `projectDocs/` para asegurar la alineación con la arquitectura actual.
2. **Validación de Skills**: Verificar las skills disponibles en el catálogo para evitar tareas manuales.
3. **Chequeo de Integridad**: Confirmar que los archivos de configuración (`tauri.conf.json`, `Cargo.toml`, `package.json`) no violan los presupuestos de RAM.

## 3. Uso Prioritario de Skills

* **Mandato de Ejecución**: Está estrictamente prohibido realizar tareas manuales que puedan ser ejecutadas mediante una Skill.
* **Uso de Skills Específicas**: Debes usar prioritariamente `kora-svelte-tauri-architect` para el Shell y `kora-sqlite-rust-engine` para la persistencia.
* **Documentación de Skills**: Al final de cada tarea, debes indicar qué skills fueron utilizadas y por qué.

## 4. Estándares Técnicos y de Código

* **Stack Tecnológico**:
    * **Backend**: Rust para el Kernel y Native Bridge (PTY real).
    * **Frontend**: Svelte 5, TypeScript y Tailwind CSS v4 (Zero-Runtime).
    * **Base de Datos**: SQLite gestionado asíncronamente desde Rust.
* **Híbrido por Diseño**: El Terminal es un proceso PTY nativo; queda prohibido el uso de terminología de "simulación".
* **Convenciones de Git**: Uso obligatorio de **Conventional Commits** (ej: `feat(kernel): implement pty bridge`, `fix(ui): adjust bento borders`).
* **Testing**: TDD obligatorio. Pruebas unitarias en Rust (`cargo test`) y test de componentes en Svelte.

## 5. Seguridad y Gobernanza (Ring Model)

* **Confianza Cero**: Valida todas las entradas y salidas del Native Bridge.
* **Integridad Criptográfica**: Implementa la fórmula de encadenamiento SHA-256 para cada evento en el log de auditoría:
    $$H_n = \text{SHA256}(Event_n + H_{n-1})$$
* **Aislamiento de Rutas**: Prohibido el uso de rutas absolutas. Todas las operaciones de archivo deben ser relativas a `/knowledge` o `/workspace`.
* **Security Gates (Ring 0)**: Cualquier acción de impacto crítico debe disparar una interrupción síncrona y esperar la aprobación explícita del usuario.

## 6. Ciclo de Vida de Desarrollo por Fases

El desarrollo debe avanzar estrictamente fase por fase con los siguientes controles:

1. **Análisis Inicial**: Lectura de reglas y documentos pertinentes antes de la fase.
2. **Refactorización y Ajuste**: Realizar cambios buscando la alineación total con la documentación.
3. **Testing y Verificación**: Ejecución de pruebas y validación de que nada ha quedado incompleto.
4. **Aprobación Ring 0**: Presentar reporte de cierre y detenerse. **No avanzar a la siguiente fase sin aprobación del usuario.**

## 7. Presupuesto de Rendimiento (Lite Mode)

* **RAM Target**: El consumo del sistema en reposo debe ser **inferior a 150 MB**.
* **Optimización**: Uso de patrones "Zero-Copy" para fragmentos de 50KB y aceleración por GPU para la interfaz Bento.
