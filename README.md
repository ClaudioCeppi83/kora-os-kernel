# 🌌 KORA OS: Cyber-Industrial Operating System

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Version](https://img.shields.io/badge/Version-v0.9.0--beta-yellow.svg)]()
[![Kernel](https://img.shields.io/badge/Kernel-Rust%201.75%2B-orange.svg)]()

KORA OS is a high-performance, hybrid-stack operating system designed for industrial-grade AI orchestration and secure data governance. Built on a foundation of Rust and Svelte 5, it implements a zero-trust architecture across four security rings.

## 🏗️ System Architecture

```mermaid
graph TD
    subgraph "Ring 0: Kernel (Rust)"
        K[KORA Kernel]
        V[Secret Vault]
        A[Audit Log SHA-256]
        J[Filesystem Jail]
    end

    subgraph "Ring 1: Governance"
        G[Agency Manager]
        D[Multi-Tenant DB]
    end

    subgraph "Ring 2: Drivers"
        P[PTY Manager]
        W[Watch Driver]
    end

    subgraph "Ring 3: User Space (Svelte 5)"
        S[Shell UI]
        E[Eco-Mode Engine]
    end

    K <--> G
    G <--> S
    K --> A
    K --> V
    K --> J
```

## 🛠️ Tech Stack

- **Core Engine**: Rust (Tauri 2.0 / Tokio)
- **Frontend**: Svelte 5 (Runes) / Tailwind CSS v4
- **Persistence**: SQLite (sqlx) with WAL mode
- **AI Orchestration**: OpenClaw Engine (open-source LLM runtime)
- **Security**: SHA-256 Chaining, Memory-Mapped Zero-Copy RAG, Path Jailing

## 🚀 Quick Start (Lite Mode)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/ClaudioCeppi83/kora-os-kernel.git
   cd kora-os-kernel
   ```

2. **Install dependencies**:
   ```bash
   pnpm install
   ```

3. **Launch in Debug Mode**:
   ```bash
   pnpm tauri dev
   ```

## ⚡ Performance Targets
- **Cold Start**: < 2s (Ready to interaction)
- **Memory Footprint**: < 150MB (Idle / Suspended AI)
- **RAG Efficiency**: Zero-Copy via `memmap2`

---

## 🛡️ Security Protocol
Vulnerabilities should be reported via encrypted channels as specified in [SECURITY.md](SECURITY.md).

## 📄 License
KORA OS is licensed under the **Apache License 2.0**. See the [LICENSE](LICENSE) file for details.
