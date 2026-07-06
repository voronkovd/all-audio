# Security Policy

## Supported versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

Security fixes are provided for the latest release in the 0.1.x line. Older pre-release builds and untagged commits are not officially supported.

## Reporting a vulnerability

If you discover a security vulnerability in All Audio, please report it responsibly.

**Do not** open a public GitHub issue for security vulnerabilities.

Instead, use [GitHub Private Security Advisories](https://github.com/voronkovd/all-audio/security/advisories/new)
or contact the maintainers through a private channel listed in the repository profile. Include:

- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You should receive an acknowledgment within **72 hours**. We will work with you to understand and address the issue.

Please allow reasonable time for a fix before any public disclosure. We aim to release a patch as soon as possible for confirmed issues affecting supported versions.

## Scope

This policy covers:

- The All Audio desktop application (Rust backend and web frontend)
- Official release artifacts built by the project CI

Out of scope:

- Vulnerabilities in FFmpeg or FFprobe themselves (report those upstream)
- Issues that require physical access to an unlocked machine
- Social engineering attacks

## Safe disclosure

We appreciate researchers who:

- Avoid accessing or modifying other users' data
- Do not perform denial-of-service attacks against project infrastructure
- Give us time to remediate before public disclosure

We will credit reporters in the release notes when a fix is published, unless you prefer to remain anonymous.
