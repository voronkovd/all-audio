# Contributing to All Audio

Thank you for your interest in contributing to All Audio!

## Getting started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- `curl`, `unzip`, and `tar` (for `npm run fetch-ffmpeg`)

Bundled FFmpeg is **not** committed to git. Run once after cloning:

```bash
npm run fetch-ffmpeg
```

### Build and run

```bash
git clone https://github.com/voronkovd/all-audio.git
cd all-audio
npm install
npm run tauri dev
```

### Production build

```bash
npm run tauri build
```

### Checks before opening a PR

Run all of the following and ensure they pass without warnings:

```bash
# Frontend
npm run check

# Download bundled FFmpeg (required once per platform)
npm run fetch-ffmpeg

# Rust
cd src-tauri
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo check
```

## Reporting bugs

Use the [Bug Report](.github/ISSUE_TEMPLATE/bug_report.yml) issue template and include:

- Operating system and version
- All Audio version (or commit hash)
- FFmpeg version (`ffmpeg -version`)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages

Do **not** include private audio files in public issues. Provide a minimal synthetic example when possible.

## Feature requests

Use the [Feature Request](.github/ISSUE_TEMPLATE/feature_request.yml) issue template. Describe the problem you are trying to solve and why the feature would benefit other users.

## Pull requests

1. Fork the repository and create a branch from `main`.
2. Keep changes focused — one logical change per PR when possible.
3. Follow existing code style and conventions.
4. Update documentation if behavior changes.
5. Add or update tests for Rust logic when applicable.
6. Ensure all checks pass (see above).
7. Fill out the pull request template completely.

### Code style

**Rust**

- Use `cargo fmt` for formatting.
- Prefer `Result` over `panic!` / `unwrap()` in production code.
- Run `cargo clippy -- -D warnings` and fix all warnings.
- Keep public API changes minimal and intentional.

**TypeScript / Svelte**

- Use existing component patterns (Svelte 5 runes).
- Add i18n strings for all user-visible text in `src/lib/i18n/locales/`.
- Run `npm run check` before submitting.

### Commit messages

Write clear, concise commit messages in the imperative mood:

- `fix: handle missing FFmpeg on Windows`
- `feat: add progress reporting for conversion`
- `docs: update README installation section`

## Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.

## Questions

Open a [Discussion](https://github.com/voronkovd/all-audio/discussions) or an issue labeled `question` if you are unsure about an approach before investing significant effort.
