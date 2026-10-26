# 18 — Development Environment (Windows-first)

## Tools (future source repo)
- Git, Rust stable toolchain `rustup`, Rustfmt/Clippy, OS C/C++ build prerequisites for crates/native UI.
- Platform SDK toolchains required by eframe/wgpu or Tauri once ADR-0001 selected.
- VS Code/terminal + Claude Code and Codex. No third agent needed.
- Node.js/npm for **npx skills and REA setup only**; native Rust app does not need Node runtime unless a Tauri UI is chosen.
- Local SQLite; disposable PostgreSQL/MySQL instances for integration; authorized Oracle 19c test instance.

## Start (PowerShell)
```powershell
# From the extracted repo root
Get-ChildItem . -Force
Get-Content .\AGENTS.md -TotalCount 40
Get-Content .\SKILLS.md -TotalCount 40
npx skills ls -g -a claude-code
npx skills ls -g -a codex
rustc --version
cargo --version
```

The blueprint contains no Rust source yet; `cargo build`/`cargo test` will not work until scaffold created. Do not interpret this as a failure of delivered docs.

## Suggested future source bootstrap (after M0 ADR)
```powershell
# Example only — performed by coding agents after ADR approval
cargo new --lib crates/domain
cargo new --lib crates/driver-api
cargo new --lib crates/app-core
cargo new --lib crates/driver-oracle
# Then define workspace Cargo.toml, formatting/lints, CI and tests.
```

## Secret handling for developer machines
- Do not commit `.env`, credentials, wallets, private certs or real DB URLs containing secrets.
- Document environment variable **names only** in future `.env.example`; use OS secret manager or scoped test account.
- Oracle server is external dependency and may be behind VPN; configure test runner intentionally, do not assume everyone has access.
- Avoid uploading REA captures and production screenshots containing client data.

## Standard build checks (once source exists)
```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
# Additional integration checks require configured test databases
```

## Platform concerns
- Windows PowerShell and symlink restrictions: use `--copy` for npx skills if necessary.
- Native UI choices require accessibility testing at multiple DPI scales, not just successful compilation.
- If Tauri chosen, runtime OS WebView versions affect binary size and UX; document additional prerequisites honestly.
