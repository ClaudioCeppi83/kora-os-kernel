---
name: kora-sqlite-rust-engine
description: Data engine patterns for asynchronous SQLite management in Rust. Handles Zero-Copy RAG, immutable SHA-256 auditing, and Session Vaulting for KORA OS.
license: Complete terms in LICENSE.txt
---

# KORA SQLite-Rust Data Engine

To manage the Core Data, Knowledge Index, and Audit Logs of KORA OS securely, implement the following data engine patterns:
1. Initialize the `kora_kernel.db` with strict asynchronous constraints.
2. Manage file segmentation (50KB chunks) using Zero-Copy memory patterns in Rust.
3. Write immutable audit logs to JSONL files using cryptographic chaining.
4. Handle context snapshots for OpenClaw's Session Vaulting.

**Stack**: Rust + sqlx (or rusqlite) + SQLite + SHA-256 Cryptography

## Security & Integrity Guidelines

VERY IMPORTANT: KORA OS operates on a "Zero Trust" model. 
- Never allow raw SQL queries from the UI or Ring 3 (OpenClaw). All queries must be parameterized.
- Always validate the `agency_id` in every transaction to enforce Multi-Tenancy Isolation. No data should leak between business units.
- Absolute paths are strictly forbidden. All file operations must be relative to the project root.

## Quick Start

### Step 1: Database Operations
Use Rust's asynchronous runtime to interact with `kora_kernel.db`. Keep queries lightweight to ensure instant retrieval of RAG metadata.

### Step 2: Zero-Copy RAG Management
When the Watch-Driver triggers an `INDEX_REQUEST`, segment files into 50KB chunks with 10% semantic overlap. Pass memory references to OpenClaw instead of duplicating strings in RAM to preserve the Lite mode budget.

### Step 3: Immutable Auditing (The Black Box)
Every action must be appended to the JSONL log. You MUST enforce the following SHA-256 chaining formula:
`H_n = SHA256(Event_n + H_{n-1})`
If the hash chain breaks, immediately trigger an agency lockdown.

### Step 4: Session Vault Snapshots
Persist OpenClaw's context state to the disk at critical milestones. If the OpenClaw sub-process crashes, use these snapshots to perform a transparent re-injection (Auto-Healing).

## Reference

- **KORA Data Model Specs**: Refer to `03_KORA_DATA_MODEL_AND_PERSISTENCE.md` and `07_KORA_SECURITY_AND_AUDIT.md`.
