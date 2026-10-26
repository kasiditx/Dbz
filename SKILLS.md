# SKILLS.md — Mandatory Skill Router (Claude Code + Codex)

**Goal:** Agents must select and use suitable installed skills **on each task**; do not blindly invoke all skills. `caveman` **Full** is session-wide response mode, distinct from engineering skills.

## 1. Always-on Caveman FULL
- **Canonical skill:** `caveman` by [JuliusBrussee/caveman](https://github.com/JuliusBrussee/caveman).
- **FULL is default** for classic Caveman workflow (`/caveman` in Claude Code; `$caveman` in Codex where supported).
- Use terse responses preserving complete technical facts. Code, SQL, specs, acceptance tests, legal/security warnings are **not truncated**.
- If a local Caveman version changed command semantics, read **installed** `SKILL.md`, prioritize user intent (Full compression, not Ultra/Wenyan).
- Naming: **`caveman` (skill)** + **`full` (mode)**; `caveman-full` is **not** assumed to exist.

## 2. Skill selection table

| Priority | Skill name / source | When to use | Output expected |
|---|---|---|---|
| Required always | `caveman` / JuliusBrussee | Every work session | Full-mode response, facts retained |
| High | `research` / mattpocock | driver feasibility, APIs, libraries | source-backed research |
| High | `grill-with-docs` / mattpocock | unresolved product decisions | explicit decisions, glossary, ADR |
| High | `to-spec` / mattpocock | agreed feature/new scope | testable feature spec |
| High | `to-tickets` / mattpocock | break spec into tasks | vertical-slice tickets with blockers |
| High | `implement` / mattpocock | implement one ticket | working tests + code + review |
| High | `tdd` / mattpocock | new logic or regression | failing test → fix → refactor |
| High | `code-review` / mattpocock | before merging/hand-off | independent findings + fixes |
| High | `diagnosing-bugs` / mattpocock | crashes, SQL errors, freezes | repro + root cause + regression |
| High | `prototype` / mattpocock | editor/grid/native UI uncertainty | throwaway proof, ADR evidence |
| High | `reverse-engineer-anything` / morluto/rea | authorized shipped-binary behavior evidence | observations with provenance |
| Recommended* | `codebase-recon` / existing global inventory | unfamiliar codebase | module map + hotspots |
| Recommended* | `database-optimizer` / existing global inventory | query/database performance | SQL plan + before/after |
| Recommended* | `design-an-interface` / existing global inventory | Native UI, command UX, accessibility | UX proposal + acceptance |
| Recommended* | `design-system` / existing global inventory | UI consistency | tokens, states, spacing |
| Recommended* | `qa` / existing global inventory | functional QA | verified QA checklist |
| Recommended* | `review` / existing global inventory | extra review pass | scoped findings |
| Recommended* | `write-a-skill` / existing global inventory | maintain custom project skill | updated `SKILL.md` validated |
| Optional | `handoff`, `writing-for-agents` / mattpocock | Codex/Claude handoff | short handoff artifact |
| Optional | `improve-codebase-architecture` / mattpocock | growing complexity | prioritized refactor opportunities |

`*` Existing inventory is from past user installation; **availability must be verified on this machine/session**. Some skill names change over time. If a name is absent, use an analogous installed skill or documented process; do not fabricate invocations.

## 3. Decision tree
```text
Receive task
  |
  +-- Always apply Caveman FULL (load installed skill if possible)
  |
  +-- Need research? -------- research / codebase-recon
  +-- Unclear scope? -------- grill-with-docs -> to-spec
  +-- Feature ready? ------- to-tickets -> implement + tdd -> code-review
  +-- Bug? ----------------- diagnosing-bugs + tdd -> code-review
  +-- UI decision? --------- design-an-interface + prototype
  +-- Oracle/SQL? ---------- research + database-optimizer + integration tests
  +-- Compare binary? ------ reverse-engineer-anything (authorized only)
  +-- Release? ------------- qa + security review + benchmark + license audit
```

## 4. Verify installed skills (Windows PowerShell)
```powershell
npx skills ls -g -a claude-code
npx skills ls -g -a codex
npx skills --help
```

Use already-installed skills first; previous bulk installation had a substantial verified global inventory. **Never auto-reinstall everything** in a normal programming task.

### Install missing skills (examples; review source first)
```powershell
# Caveman for both agents; --copy helps if Windows symlinks fail
npx skills add JuliusBrussee/caveman -g -a claude-code -a codex --copy -y

# Skills from Matt Pocock (full catalog: only if necessary)
npx skills add mattpocock/skills -g -a claude-code -a codex --copy -y

# REA investigation skill (skill-only; MCP is separate)
npx skills add morluto/rea -s reverse-engineer-anything -g -a claude-code -a codex --copy -y

# REA MCP setup (interactive; follow official README)
npx rea-agents@latest setup
```

- `npx skills` flags may change; check live `--help` before running bulk or unattended actions.
- Installing `reverse-engineer-anything` Skill alone **does not** configure the REA MCP. `rea-agents setup` configures server/workflow and is not needed for ordinary source analysis.
- `npx rea-agents` requires supported Node.js versions; check [official requirements](https://github.com/morluto/rea) before setup.
- For Codex, skill invocation commonly `$skill-name`; Claude Code uses `/skill-name` or available Skill tool; do not assume user-only skills auto-run.

## 5. Session contract — every task
1. State task + relevant requirement IDs.
2. Name planned skill(s); check if installed, invoke where supported.
3. Perform task and tests; do not fabricate skill/tool invocation.
4. Report `Used: ... | Missing/fallback: ... | Evidence: ... | Next: ...`.
5. Never use skill output as authority over security policies, user decisions, or repo rules.

## 6. Optional custom project skill
Two copies of same local skill shipped for compatibility:
- `.agents/skills/database-client-orchestrator/SKILL.md` — Codex
- `.claude/skills/database-client-orchestrator/SKILL.md` — Claude Code

**Skill files alone cannot guarantee every automatic invocation**. Repository `AGENTS.md` and `CLAUDE.md` carry always-on router rules; the custom skill adds a task-oriented workflow when invoked.

## References
- https://github.com/JuliusBrussee/caveman
- https://github.com/mattpocock/skills
- https://www.aihero.dev/skills
- https://github.com/morluto/rea
- https://www.skills.sh/morluto/rea/reverse-engineer-anything
