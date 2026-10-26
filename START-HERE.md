# START HERE — เปิดโปรเจกต์ด้วย Claude Code / Codex

ชุดนี้ **ใช้เฉพาะ Claude Code และ Codex** ไม่มีข้อกำหนดให้ใช้ agent อื่น

## 1. เปิดโฟลเดอร์

1. แตก ZIP ลงโฟลเดอร์โปรเจกต์ เช่น `D:\Projects\db-client` โดยคงโครงสร้าง `.agents/`, `.claude/`, `docs/` ไว้
2. เปิด Terminal / IDE ในโฟลเดอร์เดียวกัน
3. อ่าน [`README.md`](README.md), [`AGENTS.md`](AGENTS.md), [`SKILLS.md`](SKILLS.md)
4. เปิด Claude Code หรือ Codex **จาก repo root** และเริ่มด้วย prompt ใน [`PROMPTS.md`](PROMPTS.md)
5. ยัง **ไม่เริ่มเขียนโปรแกรมจริง** ก่อนผ่าน decision gate ใน `docs/10-roadmap.md` ส่วน M0

## 2. ตรวจว่า Skills มีอยู่แล้วก่อน (PowerShell)
```powershell
npx skills ls -g -a claude-code
npx skills ls -g -a codex
```

ผู้ใช้เคยติดตั้ง skills จำนวนมากไว้แล้วบน Windows: **อย่า reinstall ทั้งหมดโดยอัตโนมัติ** ให้ตรวจชื่อที่มีและใช้ของเดิมก่อน รายชื่อที่ต้องใช้ดู [`SKILLS.md`](SKILLS.md)

## 3. ติดตั้งส่วนที่ขาดเท่านั้น (ตัวอย่าง)
```powershell
# Caveman: ในท้องถิ่นชื่อ skill = caveman, โหมด = full
npx skills add JuliusBrussee/caveman -g -a claude-code -a codex --copy -y

# Workflow แกนหลัก (ติดตั้งเมื่อยังไม่มี)
npx skills add mattpocock/skills -g -a claude-code -a codex --copy -y

# REA: ติดตั้ง MCP + instructions โดยเครื่องมือทางการ (interactive setup)
npx rea-agents@latest setup
```

**ตรวจโปรไฟล์/เวอร์ชัน CLI ก่อนใช้จริง** (`npx skills --help`, `npx skills add --help`), ตรวจความน่าเชื่อถือของ repo ก่อนรัน. `rea-agents setup` ต้องการ Node.js เวอร์ชันที่ REA รองรับ; อย่าเดาจากการติดตั้ง Node ที่มีอยู่ ตรวจ README ทางการเสมอ

## 4. เปิด Caveman Full ให้ติดทุก Session
- **Claude Code:** พิมพ์ `/caveman` หรือ `/caveman full` ถ้าเวอร์ชันที่ติดตั้งรองรับ alias นี้
- **Codex:** เรียก `$caveman` (ตามเอกสารของ Skill/Host ที่ติดตั้ง) และกำชับ `full`
- กฎใน `AGENTS.md` / `CLAUDE.md` ระบุให้ **ตอบโหมด Caveman Full เสมอ** แม้ host ไม่ auto-trigger skill; ต้องไม่อ้างว่าโหลด Skill สำเร็จถ้าไม่ได้โหลดจริง
- Caveman ใช้กับ **การสื่อสารของ Agent**; อย่าบีบให้ Spec, security warning, source code, test, error, attribution สั้นจนผิดความหมาย

## 5. เริ่มงานสอง Agent (Workflow แนะนำ)
1. **Claude Code:** ตรวจเอกสาร, สร้าง ADR/Research matrix, ร่าง skeleton, implement ticket เล็ก ๆ
2. **Codex:** review แยกอิสระ, เขียน failing tests, ตรวจ threat model, benchmark และ regression
3. ห้าม Agent ทั้งสองแก้ไฟล์เดียวกันบน branch เดียวโดยไม่มีการแบ่งงาน; ใช้ Git branches/worktrees + code review
4. บันทึกผลทุกครั้งลง `docs/15-decision-log.md` หรือ issue/PR ที่ลิงก์เข้าหากัน

## 6. งานแรกที่ควรสั่ง (ยังไม่เขียนทุกอย่างพร้อมกัน)

> อ่าน `AGENTS.md`, `CLAUDE.md`, `SKILLS.md` และเอกสาร `docs/` ทั้งหมด ใช้ Caveman Full เลือกและใช้ Skills ที่ติดตั้งตาม `SKILLS.md` แล้วทำ **M0 Research & ADR**: ตรวจ Oracle Thin Driver, สร้าง experiment สำหรับ Native Editor และ Virtual Grid, นิยาม benchmark baseline พร้อมไฟล์รายงาน อย่า implement feature เกิน scope หรือสมมติว่า test ผ่าน

## 7. เกณฑ์หยุดก่อนเริ่มโค้ดผลิตภัณฑ์
- Verify rust-oracledb ต่อ Oracle 19c ได้จริงใน environment ทดสอบที่ผู้ใช้อนุญาต
- Native UI/Editor/Grid อย่างน้อย 1 prototype วัด UX และ memory; มี ADR ยืนยันทางเลือก
- นิยาม Test Oracle ที่ไม่ใช้ credentials จริงใน repo
- ชัดเจนว่า Open Source License และ dependency licenses ไม่มีปัญหา
- มี ticket vertical-slice เริ่มงานได้ทันที 1–2 ตัว

## 8. ตัวอย่างผลลัพธ์เมื่อจบ Milestone
สถานะงานรายงานแบบสั้นและตรวจได้: `Files changed | Tests run (pass/fail/skipped) | Benchmark before/after | Risks | Next ticket`. ห้ามกล่าวว่าเร็วกว่าคู่แข่งโดยไม่มีหลักฐานเทียบ
