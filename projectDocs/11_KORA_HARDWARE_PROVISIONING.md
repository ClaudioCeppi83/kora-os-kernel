# 11. Protocolo de Provisión e Inicialización de Hardware

## 1. Filosofía de "Arranque en Frío" (Cold Start)

El proceso de provisión no se considera una instalación de software, sino un **Aprovisionamiento de Hardware de Confianza**. KORA OS toma control total de los recursos para garantizar que el Kernel (Ring 1) sea el único mediador entre el usuario y el silicio.

## 2. Fase I: Hardware Handshake e Integridad

Al iniciar la ISO, el sistema ejecuta automáticamente "The Doctor" para verificar el entorno:

* **Detección de TPM 2.0**: Obligatorio para el almacenamiento de llaves en el Secret Vault.
* **Validación de RAM**: El sistema reserva el espacio necesario para asegurar un consumo en reposo **< 150 MB**.
* **Secure Boot Check**: Verificación de la firma del Kernel antes de cargar los drivers nucleares.

## 3. Fase II: "The Sacrifice" (Cifrado y Borrado)

KORA exige soberanía total sobre el almacenamiento. No se permite el arranque dual en modo Pro para evitar fugas de datos:

* **Borrado Seguro**: Sobrescritura de la tabla de particiones existente.
* **Cifrado LUKS/AES-256**: Configuración obligatoria de la partición de datos.
* **Entropy Generation**: El usuario debe realizar movimientos aleatorios con el cursor o teclear caracteres para generar la entropía necesaria para las llaves maestras.

## 4. Fase III: Identity Genesis (Ring 0 & Agencia Raíz)

En esta etapa se crea el sistema nervioso del nodo:

* **Sello del Anillo 0**: El usuario define la contraseña maestra que cifra el Secret Vault.
* **Creación de la Agencia Raíz**: Inicialización de la primera estructura simétrica de directorios (`/knowledge`, `/workspace`).
* **Primer Log de Auditoría**: Generación del Hash inicial ($H_0$) que servirá de semilla para toda la cadena inmutable.

## 5. Especificación Visual del Provisioner (Referencia Bento Box)

La interfaz durante la provisión mantiene la estética **Cyber-Industrial**:

* **Splash Screen**: El logo "Reactor Core" late en **Oro KORA (#D4B235)** sobre un fondo **Obsidian Black (#121212)**.
* **Efecto Scanline**: Activo al 3% de opacidad para reforzar la sensación de monitor industrial.
* **Terminal Log**: Un panel lateral muestra en tiempo real la carga de drivers de Rust mediante **JetBrains Mono**.
* **Progress Bar**: Basada en gradientes de **Rust Red (#C23B22)** a **Oro KORA** según el avance del particionado.
