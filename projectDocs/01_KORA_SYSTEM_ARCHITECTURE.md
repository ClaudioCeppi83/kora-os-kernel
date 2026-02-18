# 01. Arquitectura de Sistema y Flujo de Datos

## 1. Stack Técnico: La Integración de OpenClaw

KORA OS encapsula a OpenClaw en una estructura de cuatro capas para garantizar una autonomía sin riesgos:

### Capa 1: Interfaz (The Shell)

Construida con **Svelte + Tailwind** y empaquetada en **Tauri**, esta capa ofrece un renderizado sin runtime y acceso nativo al sistema, eliminando la latencia de los web wrappers tradicionales. Captura intenciones y renderiza terminales reales.

### Capa 2: Transporte (The Native Bridge)

Redefinido como un **Bus Binario en Rust**, gestiona la comunicación de baja latencia entre el UI-Frontend y el PTY-Backend. No es un simple túnel de texto, sino un gestor de estado concurrente.

### Capa 3: Orquestador (The Memory Manager)

Segmenta los datos para OpenClaw. Evita que el bot se sature o alucine al procesar archivos masivos de la agencia.

### Capa 4: Ejecutor (The Encapsulated Agent / OpenClaw)

Aquí reside el núcleo de OpenClaw. Se ejecuta como un subproceso restringido (low-privilege) dentro del entorno de KORA. Su función es el procesamiento lógico y el uso de herramientas autorizadas.

## 2. Flujo de Autonomía Segura

1. **Razonamiento**: OpenClaw analiza la tarea y propone un plan en el Ring 3.
2. **Interceptación**: El Kernel de KORA (Ring 1) recibe la propuesta a través del Native Bridge.
3. **Validación**: KORA verifica que las rutas sean relativas y que los drivers invocados estén permitidos.
4. **Ejecución**: Si la acción es crítica, se solicita la aprobación del Ring 0. Si es rutinaria, el Kernel ejecuta vía Drivers (Ring 2).
5. **Auditoría**: Cada paso del pensamiento y acción de OpenClaw se registra en el log inmutable con encadenamiento de hashes.

## 3. Seguridad por Diseño (Isolation Protocol)

* **Native Sandboxing**: OpenClaw no tiene acceso directo a la red o al sistema de archivos del host; KORA actúa como su proxy obligatorio.
* **Gobernanza de Memoria**: La persistencia de la tarea se asegura mediante **Snapshots de Contexto** guardados en el host por KORA, permitiendo reiniciar el proceso de OpenClaw sin perder el progreso.
