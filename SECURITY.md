# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| v0.9.x  | :white_check_mark: |
| < v0.9  | :x:                |

## Reporting a Vulnerability

We take the security of KORA OS seriously. If you discover a vulnerability, especially one related to the **Security Rings** or **Filesystem Jail**, please follow this protocol:

1. **Do not open a public issue.**
2. Report the vulnerability via an encrypted channel (PGP keys provided upon request to security@kora-os.org - *Placeholder*).
3. Provide a detailed technical breakdown, including proof-of-concept if possible.

## Response Timeline
- **Initial Acknowledgement**: Within 24 hours.
- **Triage & Classification**: Within 72 hours.
- **Remediation**: Priority depends on the affected Security Ring. Ring 0/1 vulnerabilities are handled with maximum urgency.

## Security Architecture Reference
KORA OS utilizes:
- **Ring 0**: Kernel integrity and Vault access.
- **Ring 1**: Data isolation and Governance.
- **SHA-256 Chaining**: To detect log tampering.
- **Zero-Copy RAG**: To prevent data leaks into unprotected memory regions.
