# AGENTS.md — Repository-wide instructions (Codex + all coding agents)

> **Applies to entire repository.** `CLAUDE.md` provides Claude-specific guidance. These rules apply to project work in addition to user instructions.

## Mission & authority
Build **original**, open-source, cross-platform, lightweight Database Client. Oracle is a first-class V1 target. Respect docs under `docs/`. **Claude Code + Codex only.** Never pretend a tool, skill, test, or repository has been accessed when it has not.

## Mandatory communication mode: Caveman FULL
- At start of session, check installed `caveman` skill and activate **Full** when supported (`/caveman` in Claude Code; `$caveman` in Codex according to host setup).
- Default responses: concise **Thai** (user language), technical facts precise, no filler; retain exact SQL, error strings, identifiers, warnings, caveats.
- Applies to conversation and progress updates. **Do not compress** final specs, security notices, legal obligations, tests, code, commit/PR bodies at expense of clarity.
- Never claim Caveman was loaded if only fallback style instructions were applied. If skill missing, say so once in kickoff, apply equivalent response style, and follow `SKILLS.md`.

## Skill activation contract (EVERY prompt)
1. **Read `SKILLS.md`** at session start. Check skill availability (installed skill registry / host skill listing). Never assume installation.
2. For **each distinct user task**, select relevant installed skills using router. **Caveman FULL is always-on style preference**; use workflow skills when they fit, do not invoke every skill indiscriminately.
3. When skill is available and invoking it is supported, **actually load/invoke** it, not just mention its name. Skills marked user-only must be explicitly invoked by a user or handled by an approved equivalent workflow; **do not claim they ran without invocation**.
4. If unavailable, note `Skill unavailable: NAME` and perform the documented fallback process. Do not auto-install unreviewed tools, run arbitrary remote scripts, or stop unrelated work.
5. Log selected skills + outputs in task note or final report, abbreviated in Caveman Full style.

## Routing (summary; full table in SKILLS.md)
| Need | Prefer (if installed) | Fallback |
|---|---|---|
| Understand unfamiliar repo | `codebase-recon`, `research` | source and docs inspection |
| Unclear product requirement | `grill-with-docs`, `to-spec` | write ADR and testable requirements |
| Slice spec into work | `to-tickets` | `docs/11-backlog.md` |
| Implement changes | `implement`, `tdd` | red-green-refactor |
| Find/fix bug | `diagnosing-bugs`, `diagnose` | reproduce, test, fix |
| Review | `code-review`, `review`, `code-reviewer` | independent diff review |
| UI/UX | `design-an-interface`, `design-system`, `prototype` | figma-free UI prototype + critique |
| Data/SQL performance | `database-optimizer`, `qa` | EXPLAIN + measured regression |
| Shipped binary behavior | `reverse-engineer-anything` + REA MCP | documented black-box observation |
| Session handoff | `handoff`, `writing-for-agents` | concise handoff in issue/ADR |

## Documentation order
- Start: `README.md` → `docs/01-prd.md` → `docs/02-scope.md` → relevant feature spec → ADR → current ticket.
- Docs must stay synchronized with changes. Source of truth for scope is `docs/02-scope.md`; decisions in `docs/15-decision-log.md` + `docs/adr/`.
- Ambiguity with cost/security implications: document assumption + reversible decision first; request decision if essential. Do not silently expand scope.

## Implementation principles
- Rust core independent from UI and from any provider-specific UI state. Drivers behind explicit traits/adapters.
- Database metadata browsing uses **lazy loading**. Grid uses **server-side paging + UI virtualization**. No fetching whole table into memory.
- Production-safe by default: no implicit COMMIT, no unreviewed destructive DDL/DML, staged mutations, no plaintext secret storage.
- Oracle SQL/PLSQL must never be forced into PostgreSQL/MySQL parser rules; split dialect handling and bind parameter behavior.
- Async calls must be cancellable when driver supports it; document limitations honestly.
- Errors must preserve actionable detail without leaking DB credentials or query bind values in logs.
- Keep core CLI/automation-ready; no need to build MCP server in MVP.
- New dependency: document license, MSRV, size, maintenance, CVEs, release signing impact.

## Required engineering workflow
1. Identify related requirement IDs (`FR-*`, `OR-*`, `NFR-*`) and acceptance tests.
2. Small vertical-slice ticket, failing test first for behavior changes.
3. Implement minimum code and unit/integration tests; use mocks where DB unavailable.
4. If Oracle-specific, test against a permitted Oracle instance; if unavailable report `NOT TESTED: live Oracle` prominently.
5. Run formatting, linting, tests; record exact commands and failures. Do not say `passed` without running.
6. For performance-sensitive changes, baseline then compare on identical machine and dataset.
7. Independent code review; update specs/ADRs, sample screenshots or logs **with sensitive values redacted**.

## Safety + licensing
- REA only on software you may lawfully inspect. Prefer DBeaver Community source/docs over binary decompilation. **Do not copy proprietary code, assets, branding, icons, or commercial-only features' implementation.** Behavior specs must be independently described and implemented.
- Do not access real production DB without authorization; never export real production data in a demo, benchmark, LLM prompt, issue, or test fixture.
- No third-party binary patching, DRM/license/auth bypass, credential extraction, or network MITM against systems you do not control.
- Maintain NOTICE/license obligations, SBOM and provenance in release workflow; see `docs/13-license-and-clean-room.md`.

## Work discipline
- Never rewrite architecture wholesale without ADR and measured justification.
- Never mix unrelated refactors with bug fixes. Small PRs, clear acceptance criteria and rollback path.
- Never use `cargo test` failure as permission to ignore failing tests; diagnose.
- No fabricated screenshots, benchmarks, issue links, completed tasks, source inspections, or release versions.
- For code generation, use explicit error types; preserve source compatibility or document migration.

## Report at end of each coding task
`Scope | Skills used / missing | Files | Tests (run/not run) | Perf (measured/not measured) | Security/Oracle risks | Next ticket`.
