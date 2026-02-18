# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0-beta] - 2026-02-18

### ✨ Added
- **Ring 0: Extreme Optimization**: Parallelized boot sequence (< 2s cold start).
- **Zero-Copy RAG**: Integration of `memmap2` for efficient large-file indexing.
- **Eco-Mode**: Adaptive UI throttling based on CPU telemetry.
- **Secret Vault**: Ephemeral environment variable management and PII scrubbing.
- **Governance**: Multi-tenant `agency_id` implementation and context switching.
- **Audit Log**: Immutable SHA-256 chaining for all Ring 1/0 events.
- **Shell UI**: Bento-box layout with Xterm.js integration and scanline effects.

### 🛡️ Fixed
- Race condition in kernel-ready signaling (Splash Screen fixes).
- Symbol collision in Tauri commands during binary hardening.
- PTY buffer overflow under high-frequency interaction.

### 🚀 Performance
- Cargo LTO and symbol stripping enabled for minimal binary size.
- SQLite WAL mode and PRAGMA optimizations.
