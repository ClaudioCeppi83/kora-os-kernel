# 08. Guía de Despliegue y Configuración (Pro vs. Lite)

## 1. Filosofía de Despliegue

KORA OS se distribuye como una imagen de sistema operativo inmutable. El objetivo es que el usuario no realice instalaciones manuales de dependencias, sino que el entorno venga pre-optimizado para el Native Bridge y la ejecución de OpenClaw desde el primer arranque.

## 2. Requisitos de Compilación (Build Environment)

Para generar la imagen inmutable de KORA Pro, el entorno de construcción debe contar con:

* **Rust Toolchain**: stable (para compilar el Native Bridge y el Kernel).
* **Node.js 20+ & pnpm**: Para la transpilación de la interfaz Svelte/Tailwind.
* **Tauri CLI**: Para el empaquetado de la aplicación híbrida.

## 3. Aprovisionamiento de KORA Pro (Hardware Appliance)

El nodo maestro requiere una instalación permanente en almacenamiento de alta velocidad para gestionar el *Knowledge Tree* de múltiples agencias.

### Pasos de Instalación

1. **Selección de Imagen**: Descargar la ISO correspondiente a la arquitectura (ARM64 para SBCs o x86_64 para servidores).
2. **Flashing**: Utilizar herramientas de escritura de bajo nivel para volcar la imagen en el NVMe SSD.
3. **Configuración de BIOS/UEFI**:
   * **Activar TPM 2.0** para el cifrado del Secret Vault.
   * **Habilitar Secure Boot** para garantizar que solo el Kernel firmado de KORA pueda arrancar.
4. **Primer Arranque**: El sistema ejecutará automáticamente "The Doctor" (Motor de Diagnóstico) para verificar la integridad del hardware y la asignación de RAM para el Memory Manager.

## 4. Creación de KORA Lite (Live USB)

El dispositivo Lite se comporta como un entorno "Plug & Play" que arranca íntegramente en la RAM del host objetivo.

### Configuración del Dispositivo

1. **Preparación del Medio**: Se recomienda un USB 3.1 de grado industrial (mínimo 32GB).
2. **Particionado Simétrico**:
   * **Partición OS (Inmutable)**: Contiene el Kernel y el Shell de KORA.
   * **Partición Persistente (Cifrada)**: Espacio dedicado para el `/workspace` temporal y los logs de auditoría generados en modo offline.
3. **Check-out Inicial**: Conectar el USB al Hardware Pro y ejecutar `kora system checkout --target /dev/sdb` para sincronizar las identidades de los agentes y los snapshots de contexto necesarios para el viaje.

## 5. Configuración Inicial del Ecosistema

Una vez arrancado el sistema por primera vez, se deben completar los siguientes hitos de configuración:

### A. Inicialización de Agencias

```bash
kora agency create "Marketing_Agency_01"
kora agency create "Digital_Support_Restaurantes"
```
