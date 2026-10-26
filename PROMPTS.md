# PROMPTS.md — Copy/Paste สำหรับ Claude Code และ Codex

ก่อนเริ่มแต่ละงานให้เปิด agent ใน repo root; กฎ Caveman Full อยู่ใน `AGENTS.md` / `CLAUDE.md`. Prompt ภาษาไทย; แก้ชื่อ path ตามเครื่องได้.

## P0 — Bootstrap & review เอกสาร (Claude Code)
```text
อ่าน README.md, AGENTS.md, CLAUDE.md, SKILLS.md และ docs/*.md รวมถึง ADR ทั้งหมด
ใช้ Caveman Full ในคำตอบ และตรวจ/เรียกใช้ Skills ที่ติดตั้งตาม SKILLS.md
ตรวจ consistency ระหว่าง PRD, Scope, Functional Spec, Oracle Spec, Acceptance Matrix, Roadmap
ทำรายการข้อขัดแย้งกับข้อกำหนดที่ยังไม่ตัดสินใจ ห้ามแต่ง benchmark หรือ claim สิ่งที่ยังไม่ทดสอบ
สร้าง issues/tickets เฉพาะ M0 แบบ vertical slices และเขียน work plan ก่อนเริ่มโค้ด
```

## P1 — Research Oracle Driver (Claude Code)
```text
ดำเนินงาน M0 Oracle feasibility ตาม docs/04-oracle-spec.md และ docs/11-backlog.md
ใช้ research และ prototype skills ถ้ามี; Caveman Full
ศึกษาจาก rust-oracledb เอกสารทางการ ตรวจ supported server versions, bind/PLSQL,
LOB, transactions, cancel, pooling, auth/TCPS/wallet, license, limitations และ MSRV
เขียน proof-of-concept เฉพาะฐานทดสอบที่ได้รับอนุญาต โดยห้ามบันทึก credentials ลง repo
หากยังไม่มี Oracle test instance ทำ mock tests แล้วระบุ live integration = NOT TESTED
ออก driver ADR และหลักฐาน command/log redacted
```

## P2 — Native UI experiment (Claude Code)
```text
ทำ M0 Native UI prototype เท่านั้น ตาม docs/adr/0001-ui-stack.md และ docs/06-ux-ui-spec.md
ใช้ design-an-interface, prototype และ design-system ถ้าติดตั้ง; Caveman Full
ทดลอง SQL editor (highlight, completion stub, multiline) และ virtualized grid 100k rows
ทดสอบ keyboard navigation, resize, dark mode, long cells, scrolling, memory, startup
เปรียบเทียบ egui/eframe กับ Tauri alternative อย่างเป็นธรรม แล้วบันทึก ADR ก่อนเลือก stack
ห้ามสรุปว่าเครื่องมือใดเร็วกว่าโดยไม่มี baseline
```

## P3 — Implement M1 Oracle Vertical Slice (Claude Code)
```text
เลือกเฉพาะ ticket M1 ที่ยังไม่ถูก block ใน docs/11-backlog.md
ใช้ implement + tdd + code-review (หรือ fallback ตาม SKILLS.md), Caveman Full
สร้าง Rust Core/Driver boundary, Oracle connect/test connection, schema list,
parameterized SELECT, paging 100 rows, cancel best-effort และ typed errors
ทุก acceptance criterion ต้องมี test; ถ้าไม่ผ่าน/ยังทดสอบ live ไม่ได้ ต้องรายงานชัด
ไม่สร้าง features beyond M1 และไม่ทำ Git push/release โดยไม่ขออนุญาต
```

## P4 — Independent review (Codex)
```text
Review code diff ของ ticket ล่าสุดตาม AGENTS.md และ docs/03-functional-spec.md
โหลด caveman Full ถ้ามี, ใช้ code-review/review/qa หากติดตั้ง
ตรวจ Rust error handling, driver contracts, query cancellation, SQL injection, bind values,
transactions, credential leakage, incomplete paging, UI blocking และ Oracle dialect
เรียง findings ตาม severity; สำหรับแต่ละ finding ให้ไฟล์/บรรทัด/การทำซ้ำ/impact
ถ้าแก้โค้ด ให้เขียน regression test ก่อน แล้วรันเทสต์จริง
```

## P5 — Authorized UX research with REA (Claude Code / Codex)
```text
ทำเฉพาะรายการ research ที่ได้รับอนุญาตใน docs/12-rea-research-plan.md
ตรวจว่า REA MCP เชื่อมต่อจริง; เรียก reverse-engineer-anything skill เมื่อเหมาะสม
ลำดับแรกให้อ่าน DBeaver Community source/docs แล้วทดลอง UI แบบ black-box
ห้ามคัดลอก proprietary code, icons/assets, credentials, hidden APIs หรือ patch binaries
บันทึก target version, test case, expected/observed behavior, evidence และข้อจำกัด
เปลี่ยน observation เป็น acceptance test แบบอิสระ ไม่สร้างโค้ดจาก decompiled source
```

## P6 — Performance review (Codex)
```text
ใช้ docs/08-performance.md เป็นสเปก benchmark และทดสอบบนเครื่องเดียวกัน
วัด cold/warm startup, idle/active RAM, first paint, first SQL row,
100k scroll, schema expansion, cancellation behavior, CPU usage
เก็บ hardware/OS/driver/DB version/dataset/iteration/environment ทุก run
เปรียบเทียบ DBeaver/Gridline เฉพาะเมื่อทดสอบได้จริง; บอก N/A ถ้าไม่ได้
ห้ามแปลว่า query latency = client performance โดยไม่แยก network/server overhead
```

## P7 — Public beta release (Codex review + Claude execution)
```text
ตรวจ docs/16-release-and-community.md และ docs/20-definition-of-done.md
ตรวจ security, attribution, third-party NOTICE, SBOM, packages, crash reporting opt-in,
installer update checks, signature where available และ reproducible build log
ทำ checklist ผ่าน/ไม่ผ่านจริงทุกข้อ; หากไม่มีการทดสอบ Oracle อย่า release ว่า Oracle supported
```
