# CLAUDE.md — Claude Code Project Instructions

Read **`AGENTS.md` first**. It is repository-wide source of truth. Do not replace or contradict its requirements.

## Start-of-session protocol
1. Read `README.md`, `AGENTS.md`, `SKILLS.md`, `docs/02-scope.md`, relevant ticket/spec.
2. Set **Caveman Full** for user-facing responses. Load `caveman` skill if present. Slash command usually `/caveman` (Full is default); do not call `/caveman full` blindly unless installed variant supports it.
3. Discover installed skills; choose by task. If skill unavailable, use documented fallback and report accurately.
4. Check current branch and working tree before edits; avoid overwriting concurrent Codex changes.
5. Identify smallest verifiable vertical slice and exact requirements before writing code.

## Mandatory task-specific skill flow
- New feature: use `grill-with-docs` only for unresolved decisions, then `to-spec` and `to-tickets` as appropriate; development with `implement` + `tdd`; end with `code-review`.
- Architectural uncertainty: `research` + `prototype`; produce ADR and reproducible measurement.
- UI: `design-an-interface` if installed, backed by screenshots/keyboard/accessibility acceptance checks.
- Oracle/SQL: `database-optimizer` if installed; perform dialect + bind type checks and integration tests.
- REA investigation: `reverse-engineer-anything` only when authorized and observable evidence required; do not blindly decompile open-source components.
- For task handoff to Codex: `handoff` if installed, else write `docs/`/issue handoff: changed files, invariants, tests, open risks.

**Important:** Some skills (e.g. `implement`, `to-tickets`) require **explicit user invocation** or plugin-specific Skill tool invocation. If only project rules were loaded, that is not proof the skill executed. When unavailable or not invocable, follow the equivalent process and label it fallback.

## What to build first
Read `docs/10-roadmap.md` M0 → M1. Start with isolated UI editor/grid and Oracle Thin Driver POC; do not scaffold entire feature universe on the first prompt. Native egui is **candidate**, final choice depends on ADR-0001 gate.

## Do not
- Treat a markdown blueprint as a working source repository.
- Create misleading benchmarks or assert Oracle 11g compatibility with Thin driver.
- Store secrets in Rust source, tests, `sqlite` config, debug logs, screenshots or agent prompts.
- Unilaterally publish/push commits, installers or release files without user instruction.
