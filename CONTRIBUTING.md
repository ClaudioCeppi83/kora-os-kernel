# Contributing to KORA OS

First off, thank you for considering contributing to KORA OS. It's people like you that make KORA OS such a great tool.

## 📜 Code of Conduct
We follow high standards of engineering integrity. Respect the security rings and never attempt to bypass jail enforcement in production-bound code.

## 🛠️ Development Workflow
1. Fork the repository.
2. Create a feature branch from `dev`.
3. Commit your changes using the protocol below.
4. Submit a Pull Request to `dev`.

## 💬 Commit Protocol
We strictly follow **Conventional Commits**. Every commit message must have a type prefix:

- `feat:` for new features (e.g., `feat: add hardware abstraction layer`)
- `fix:` for bug fixes (e.g., `fix: resolve PTY buffer overflow`)
- `security:` for security-related changes (e.g., `security: rotate vault keys`)
- `perf:` for performance optimizations (e.g., `perf: optimize RAG chunking`)
- `docs:` for documentation updates
- `refactor:` for code restructuring without changing behavior

## 🧪 Testing Requirements
- All Rust changes must pass `cargo check` and `cargo test`.
- Frontend changes must satisfy linter requirements.
- Any security-sensitive change requires an accompanying audit log entry.

## 🏗️ Pull Request Standards
- Keep PRs focused on a single logical change.
- Link relevant issues.
- Update documentation in `/projectDocs` if architecture is affected.
