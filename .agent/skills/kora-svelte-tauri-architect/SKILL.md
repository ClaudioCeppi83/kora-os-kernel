---
name: kora-svelte-tauri-architect
description: Suite of tools and patterns for creating high-performance, zero-runtime KORA OS frontend interfaces using Svelte 5, Tauri 2.0, and Tailwind CSS. Strictly adheres to the Bento Box design system and Native Bridge IPC.
license: Complete terms in LICENSE.txt
---

# KORA Svelte-Tauri Architect

To build the hybrid Shell for KORA OS, follow these steps and patterns to ensure zero-runtime performance and deep system integration:
1. Structure the UI using the strict modular "Bento Box" layout.
2. Develop the terminal component using Xterm.js connected to a real backend PTY.
3. Establish communication using Tauri IPC (Inter-Process Communication) via binary KORA-RPC.
4. Implement synchronous UI locking for Security Gates (Ring 0 approvals).

**Stack**: Svelte 5 + TypeScript + Vite + Tauri 2.0 + Tailwind CSS v4 + Xterm.js

## Design & Style Guidelines

VERY IMPORTANT: To maintain the "Cyber-Industrial" identity of KORA OS, strictly avoid rounded, friendly interfaces. 
- Use sharp, 1px borders with low opacity (`primary/20`) for all Bento modules.
- Enforce the color palette: Background (`#121212`), KORA Gold (`#D4B235`) for active accents, and Rust (`#C23B22`) for alerts/errors.
- Implement subtle Scanlines (`rgba(212, 178, 53, 0.03)`) and optimized CSS Glow effects without penalizing the framerate (< 16ms latency).
- Typography must be dual: Space Grotesk (Display) and JetBrains Mono (Terminal/Data).

## Quick Start

### Step 1: Initialize Shell Component
Ensure Svelte is configured without a heavy runtime to respect the < 150MB RAM budget for KORA Lite. Compile components strictly.

### Step 2: Integrate Xterm.js
The terminal is not a simulation. Initialize `Xterm.js` with the WebGL addon to offload rendering to the GPU. Ensure standard STDIN/STDOUT piping to the Rust backend.

### Step 3: Implement Native Bridge IPC
Use Tauri's command system (`invoke`) to send binary payloads. Never send massive JSON strings when transferring 50KB RAG chunks; use optimized binary serialization.

### Step 4: Security Gates (System Freeze)
When the Rust Kernel sends a `RING_0_APPROVAL_REQUIRED` event, immediately apply a blur filter to the UI, change the terminal cursor to Rust (`#C23B22`), and block all inputs except the approval modal.

## Reference

- **KORA OS UI/UX Specs**: Refer to `04_KORA_UI_UX_SPECIFICATION.md` for layout mapping.
