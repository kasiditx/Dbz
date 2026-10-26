# Open-Source Database Client — Project Blueprint

> **Status:** Planning / Specification · **ชื่อโปรเจกต์:** ชื่อชั่วคราว `db-client` (ยังไม่กำหนดแบรนด์) · **อัปเดต:** 2026-10-26

ชุดเอกสารสำหรับสร้าง **Database Client ใหม่แบบ Open Source** ที่ทันสมัย ใช้ง่าย เปิดไว ใช้ทรัพยากรต่ำ และรองรับ **Oracle เป็น First-Class Database** รวมถึง PostgreSQL, MySQL/MariaDB, SQLite โดยใช้ **Claude Code + Codex** เท่านั้นในการช่วยพัฒนา

**แนวทาง:** ศึกษาฟังก์ชันและ UX จาก DBeaver, Gridline, Oracle SQL Developer และโปรเจกต์ ArtCraft; พัฒนาสถาปัตยกรรมและโค้ดของเราเอง ไม่ใช่ Fork เปลี่ยนธีม ไม่คัดลอกโค้ด/asset เชิงพาณิชย์ที่ไม่มีสิทธิ์ใช้ ใช้ REA เฉพาะกรณีที่การศึกษาพฤติกรรมโปรแกรมที่ได้รับอนุญาตให้วิเคราะห์ช่วยตอบคำถามที่ซอร์สโค้ดหรือเอกสารตอบไม่ได้

## หลักการ 7 ข้อ
1. **Oracle First-Class:** เชื่อมต่อ, รัน SQL/PLSQL, เปิด Schema, แก้ข้อมูลอย่างปลอดภัย, Transactions, LOB อย่างเหมาะสม
2. **Fast & Lean:** startup, idle RAM, interaction latency, first-result latency มีตัวเลขเป้าหมายและ Benchmark จริง
3. **Modern UX:** keyboard-first, connection switching, SQL editor ที่ดี, ตารางข้อมูลลื่น, error ชัด
4. **Core/UI Split:** Rust engine/driver แยกออกจาก Native UI; CLI/automation reuse core ได้
5. **Safe Default:** protect production; secrets ใน OS keychain, read-only, staged writes, parameter binding
6. **Open Source by Design:** license/attribution/SBOM/reproducible builds/community contribution ตั้งแต่วันแรก
7. **Evidence over claims:** คำว่า “เร็วกว่า DBeaver” ต้องมาจาก benchmark ที่เทียบงานจริงบน environment เดียวกัน

## เริ่มต้นอ่าน
1. [`START-HERE.md`](START-HERE.md) — วิธีนำชุดไฟล์ไปใช้ใน Claude Code / Codex (Windows)
2. [`AGENTS.md`](AGENTS.md), [`CLAUDE.md`](CLAUDE.md) — กติกาหลักของ Coding Agents; **Caveman Full** ทุก session
3. [`SKILLS.md`](SKILLS.md) — รายชื่อ Skills, คำสั่งตรวจ/ติดตั้ง, Skill Router
4. [`docs/01-prd.md`](docs/01-prd.md), [`docs/02-scope.md`](docs/02-scope.md) — Product Requirements + In/Out Scope
5. [`docs/03-functional-spec.md`](docs/03-functional-spec.md), [`docs/04-oracle-spec.md`](docs/04-oracle-spec.md) — Functional Spec และ Oracle Spec
6. [`docs/05-architecture.md`](docs/05-architecture.md), [`docs/06-ux-ui-spec.md`](docs/06-ux-ui-spec.md) — Architecture + UX
7. [`docs/09-testing.md`](docs/09-testing.md), [`docs/10-roadmap.md`](docs/10-roadmap.md), [`docs/11-backlog.md`](docs/11-backlog.md) — Test, Roadmap, Ticket slices
8. [`PROMPTS.md`](PROMPTS.md) — คำสั่งพร้อมคัดลอกให้ Agent เริ่มทำงาน

## เอกสารทั้งหมด
| กลุ่ม | ไฟล์ |
|---|---|
| Product | `00-vision`, `01-prd`, `02-scope`, `03-functional-spec`, `17-acceptance-matrix` |
| Oracle / Engine | `04-oracle-spec`, `05-architecture`, `14-driver-contract`, `adr/*` |
| UX / Quality | `06-ux-ui-spec`, `07-security`, `08-performance`, `09-testing`, `20-definition-of-done` |
| Delivery | `10-roadmap`, `11-backlog`, `15-decision-log`, `16-release-and-community`, `18-development-environment` |
| Research / Legal | `12-rea-research-plan`, `13-license-and-clean-room`, `19-competitive-feature-matrix` |
| Agents | `AGENTS.md`, `CLAUDE.md`, `SKILLS.md`, `PROMPTS.md`, `.agents/skills/*`, `.claude/skills/*` |

## Product milestones
- **M0 Research/ADR:** ตรวจสอบ Oracle driver, Native UI Editor/Grid prototype, แผน benchmark
- **M1 Oracle Vertical Slice:** connection + schema list + read-only SELECT + paging + error/cancel
- **M2 Core Client:** query tabs + transactions + staged edit + exports + PostgreSQL/SQLite
- **M3 Multi-DB / Polish:** MySQL/MariaDB + accessibility + perf & security hardening
- **M4 Public Beta → v1.0:** installers, licenses, contributor docs, Oracle support matrix, CI

> **หมายเหตุ:** ยังไม่มี Source Code ของผลิตภัณฑ์ในชุดนี้; ทั้งหมดเป็นเอกสารตั้งต้นให้ Agent สร้างโปรเจกต์อย่างเป็นระบบ ตัวเลขประสิทธิภาพคือ **targets**, ไม่ใช่ benchmark ที่ตรวจวัดแล้ว

## แหล่งอ้างอิงหลัก
- [REA — morluto/rea](https://github.com/morluto/rea)
- [DBeaver Community](https://github.com/dbeaver/dbeaver)
- [Gridline](https://github.com/adrianbonpin/gridline)
- [Oracle rust-oracledb](https://github.com/oracle/rust-oracledb)
- [Oracle rust-oracle (OCI fallback)](https://github.com/kubo/rust-oracle)
- [ArtCraft / PhotoCraft](https://github.com/storytold/photocraft)
- [Caveman Skill](https://github.com/JuliusBrussee/caveman)
- [Matt Pocock Agent Skills](https://github.com/mattpocock/skills)
