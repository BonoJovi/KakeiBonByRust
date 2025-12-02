# Security Policy

## Supported Versions

Security updates are provided for the latest stable release.

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Security Features

KakeiBon implements the following security measures:

- **Password Hashing:** Argon2 (industry-standard, memory-hard)
- **Data Encryption:** AES-256-GCM for sensitive data at rest
- **Database:** SQLite with prepared statements to prevent SQL injection
- **Input Validation:** Both frontend and backend validation
- **Dependency Security:** Automated Dependabot monitoring

## Security Documentation

Detailed security investigations and decisions are documented in:
- `docs/security/` - Security-related memos and analyses

## Reporting a Vulnerability

If you discover a security vulnerability in KakeiBon, please report it via:

- **GitHub Security Advisories:** https://github.com/BonoJovi/KakeiBonByRust/security/advisories/new
- **Private Issue:** Create an issue with "SECURITY" label (if you prefer not to use GitHub's advisory feature)

### What to Include

Please provide:
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Suggested fix (if any)

### Response Timeline

- **Acknowledgment:** Within 48 hours
- **Initial Assessment:** Within 1 week
- **Fix Timeline:** Depends on severity
  - Critical: Immediate action
  - High: Within 1 week
  - Medium/Low: Next planned release

### Disclosure Policy

We follow responsible disclosure:
1. Report received and acknowledged
2. Vulnerability verified and assessed
3. Fix developed and tested
4. Security advisory published with fix
5. Public disclosure after fix is available

Thank you for helping keep KakeiBon secure!
