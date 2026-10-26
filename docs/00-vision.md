# 00 — Product Vision & Principles

## One-sentence vision
**Open-source, native-feeling database workspace that makes Oracle and other SQL databases approachable without sacrificing performance or operational safety.**

## Problem
DBeaver, SQL Developer และเครื่องมืออื่นให้ฟีเจอร์ครบ แต่ผู้ใช้จำนวนหนึ่งต้องการ UI ที่เรียบกว่า, การนำทางเร็วกว่า, startup เร็วกว่า และไม่มีความรู้สึกว่าต้องเปิด IDE ขนาดใหญ่เพื่อแก้ข้อมูลไม่กี่แถว. ข้อความนี้เป็น hypothesis ไม่ใช่หลักฐาน benchmarking; validate กับผู้ใช้จริงก่อนตัดสินใจ roadmap.

## Target personas
1. **Application Developer:** สลับ PostgreSQL / Oracle / MySQL, ค้น table, inspect data, run query, export.
2. **Oracle Developer:** ดู packages/procedures, ทำ SQL/PLSQL, สำรวจ schemas, bind variables, transactions.
3. **DBA/Power User (subset):** การทำงานซ้ำด้วย keyboard, query plan, admin metadata แบบอ่านอย่างปลอดภัย. ไม่ใช่เครื่องมือ DBA แบบเต็มรูปแบบใน v1.
4. **Learner:** connection flow เข้าใจง่าย, error อ่านรู้เรื่อง, SQL preview ก่อนแก้ข้อมูล.

## User outcomes
- เชื่อมต่อ Oracle 19c ที่ได้รับอนุญาตได้ด้วย host/port/service user/pass (ตามนโยบาย network).
- หา table และเปิด 100 แถวแรกภายในไม่กี่ action โดยไม่ preload table ทั้งหมด.
- เขียน SQL หลายแท็บ พร้อม query cancel, history, safe commit/rollback.
- มองเห็นชัดว่า connection ไหนเป็น Production/Read-only.
- โปรแกรมไม่ค้างเมื่อ query นาน, expand schema ใหญ่ หรือเปิด LOB.

## Principles
- Performance is a feature with a measurable budget.
- Safety beats convenience for destructive operations.
- Prefer boring, maintainable drivers and testability over hype.
- Features must be usefully complete across a vertical slice, not decorative menu items.
- No forced telemetry, login/account, ads, or cloud service for local database use.
- OSS governance: document architecture, license, contributor paths and decisions publicly.

## Differentiators to validate
- First-class Oracle in a light Rust client, including practical SQL/PLSQL workflows.
- Modern native-feeling UI with fast keyboard navigation and large-grid virtualization.
- Driver architecture amenable to testable extensions.
- Safe staged writes/connection profile visibility by default.

## Non-goals
- Rebuild every DBeaver/SQL Developer feature in 1.0.
- Clone UI/brand/assets or emulate proprietary internals.
- Invent a replacement database server or SQL engine; this is a **client**, not an RDBMS.
- Promise better database query execution times independent of server/index/network.

## Success signals
- Repeated weekly developer use in Oracle and one additional DB type.
- First-run connection completion; weekly query actions; tasks accomplished; low crash rate.
- Meets measured NFR targets on specified baseline device; no data loss bugs in staged edits.
- Community can submit a driver or UI bugfix with docs/tests and CI feedback.
