# Dbz — T-004a: Tauri vs egui Same-Workload Comparison

ดำเนินการพัฒนาโปรเจกต์ Dbz ต่อจาก Local Commit `25711f5` โดยมี `0d4972d` เป็น Commit ก่อนหน้า

**Mission:** สร้าง Tauri 2 UI Prototype และเปรียบเทียบกับ egui/eframe 0.37 ด้วย Workload เดียวกัน เพื่อหาว่า UI Stack ใดเหมาะสมกับ Open-Source Database Client ที่ทันสมัย รวดเร็ว ใช้งานง่าย และใช้ทรัพยากรต่ำ

## 1. Mandatory Skills

อ่าน `AGENTS.md`, `CLAUDE.md`, `SKILLS.md` และเอกสารที่เกี่ยวข้อง

เปิดใช้ Caveman Full และ Invoke Skills ที่ติดตั้งจริงตามความเหมาะสม:

- `caveman` — Full Mode
- `research` — ศึกษา API และ Trade-offs
- `prototype` — สร้าง UI Experiment
- `design-an-interface` — UX/UI
- `design-system` — UI Consistency
- `tdd` — Test-Driven Development
- `qa` — Functional Validation
- `code-review` — Code Review

หาก Skill ใดไม่มีหรือเรียกไม่ได้ ให้รายงานตามจริงและใช้ Fallback

## 2. Git Safety

1. ตรวจสอบ `git status`, Branch และ Commit ก่อนเริ่ม
2. รักษา Commit `0d4972d` และ `25711f5` ไว้
3. สร้าง Branch `experiment/t004a-tauri` จาก Local HEAD ที่ถูกต้อง
4. ห้าม Reset, Force Push หรือทับไฟล์งานเดิม
5. ห้าม Push โดยไม่ได้รับคำสั่งจากผู้ใช้

## 3. Read Previous Evidence

อ่าน:

- `docs/evidence/T-003-native-ui.md`
- `docs/evidence/T-003-followup.md`
- `docs/adr/0001-ui-stack.md`
- `docs/05-architecture.md`
- `docs/06-ux-ui-spec.md`
- `docs/08-performance.md`
- `docs/09-testing.md`
- `docs/11-backlog.md`

ใช้ผลทดสอบ egui ล่าสุดเป็น Baseline และตรวจสอบ Raw Data ก่อนเปรียบเทียบ

## 4. Build Tauri Prototype

Technology:

- Tauri 2
- Rust Backend
- React + TypeScript
- Vite
- CodeMirror 6
- TanStack Virtual
- CSS / Lightweight UI Components

ใช้ Dependency Versions ที่ตรวจสอบแล้วและบันทึกไว้ใน Lockfiles

สร้าง Prototype แยกจาก Production Core และไม่แก้ Oracle POC

### Required Features

1. Modern Dark/Light Theme
2. SQL Editor พร้อม Syntax Highlighting
3. Multiple SQL Tabs
4. Mock Database Explorer
5. Virtualized Grid 100,000 และ 1,000,000 Rows
6. Keyboard Navigation
7. Resizable Sidebar และ Editor/Result Splitter
8. Status Bar
9. Thai Unicode / Long Text Support
10. Mock Query Run/Cancel โดยไม่ Block UI

ต้องมี Loading, Empty และ Error States ที่เหมาะสม

ห้ามใช้ Dev Server หรือ Debug Build เป็นตัวเลข Performance สำหรับตัดสินใจ

## 5. Same-Workload Contract

สร้าง Workload Contract กลางสำหรับ egui และ Tauri

ต้องเทียบโดยใช้:

- Window Size เดียวกัน
- Dataset 1K / 100K / 1M Rows
- จำนวนคอลัมน์ 20 Columns
- Row Height และ Column Width ใกล้เคียงกัน
- Deterministic Synthetic Data Generator
- Mixed NULL / Unicode / Thai / Decimal / Timestamp / Long Text
- SQL Script เดียวกัน
- Editor Tabs เท่ากัน
- Same Mock Query Delay
- Same Keyboard Actions
- Same Scroll Pattern
- Same Resize Pattern

ห้ามสร้าง 1,000,000 DOM Rows

ใช้ Row/Column Virtualization และตรวจสอบจำนวน DOM Elements ที่ใช้งานจริง

ถ้าบางฟีเจอร์เทียบกันไม่ได้ ต้องบันทึกความแตกต่างและผลกระทบต่อ Benchmark

## 6. Fair Performance Benchmark

เปรียบเทียบ **Final Release Builds** บนเครื่องเดียวกัน ภายใต้สภาพแวดล้อมเดียวกัน

วัด:

| Metric | Requirement |
|---|---|
| Warm Startup | 10 Runs |
| Cold Startup | 10 Runs หากวัดได้ถูกต้อง |
| Idle Memory | หลัง 60 วินาที |
| Peak Memory | ระหว่าง Scrolling |
| Idle CPU | Multi-Run |
| Input-to-App-State | p50 / p95 |
| Frame Interval | p50 / p95 |
| Window Resize | Real Mouse |
| Tab Switching | Real Input |
| Editor Typing | Large SQL |
| Grid Scrolling | 100K / 1M Rows |
| Binary Size | Release Artifact |

### Critical: Tauri Memory Accounting

นับ Memory ของ:

- Main Tauri Process
- WebView WebContent Process
- WebKit Network / Helper Processes ที่ระบุได้ว่าเป็นของ App นี้
- Other Attributable Child Processes

ห้ามนับเฉพาะ Main Process แล้วสรุปว่า Tauri ใช้ RAM ต่ำ

ระบุ PID, Process Names และวิธีเชื่อมโยง Helper Processes กับ App

รายงานทั้ง Main-Only RSS และ Attributed Process-Group RSS โดยระบุว่าการรวม RSS อาจนับ Shared Pages ซ้ำ

หากวัด Physical Footprint หรือ Memory Metric ที่ลดปัญหา Shared Pages ได้ ให้รายงานแยก

ห้ามรวม WebKit Process ของ App อื่นเข้ามาโดยไม่มีหลักฐาน

### Timing Accuracy

แยก:
- JavaScript Execution
- React Commit / Rendering
- Browser Frame Intervals
- Rust Backend Work
- OS Event Delivery
- Input-to-App-State Latency
- Input-to-Display Latency (เมื่อวัดได้)

อย่านำ `ui()` CPU Time ของ egui ไปเทียบตรง ๆ กับ React Commit Time เพราะเป็นคนละขั้นตอน

## 7. Functional Testing

ทดสอบด้วย Real OS Input เมื่อเป็นไปได้:

- SQL Typing และ Selection
- Keyboard Shortcuts
- Grid Cell Selection
- Copy/Paste
- Tab Navigation
- Sidebar Expand/Collapse
- Drag Splitter
- Window Resize
- Thai Text
- Mouse Scrolling
- Focus Transfer

ใช้ TDD สำหรับ Logic ใหม่

รัน Unit Tests, Typecheck, Lint, Rust fmt/Clippy และ Release Build

## 8. Accessibility Comparison

เปรียบเทียบ:

- Keyboard-Only Navigation
- Focus Indicators
- Screen Reader / Accessibility Tree
- Grid Accessibility
- Text Selection
- Thai IME
- HiDPI / Font Rendering

ถ้าทดสอบ VoiceOver หรือ Thai IME จริงไม่ได้ ให้ระบุ NOT TESTED

อย่าอ้างว่ารองรับ Accessibility เพียงเพราะใช้ React หรือ WebView

## 9. Comparison Report

สร้าง:

- `docs/evidence/T-004a-tauri-comparison.md`
- `benchmarks/t004a/README.md`
- Benchmark Scripts
- Raw CSV/JSON
- Hardware and Environment Metadata
- Screenshots ที่จับจากแอปจริง
- Test Reports

รายงานผลโดยใช้ตาราง:

| Metric | egui | Tauri | Notes |
|---|---|---|---|
| Startup | Measured | Measured | |
| RAM | Measured | Measured | |
| CPU | Measured | Measured | |
| Input | Measured | Measured | |
| Scroll | Measured | Measured | |
| Resize | Measured | Measured | |
| Accessibility | Verified/Gap | Verified/Gap | |
| Development Complexity | Evidence | Evidence | |

สำหรับ Metric ที่วัดไม่สำเร็จให้ใช้ NOT MEASURED ไม่ใช่ตัวเลขสมมติ

ห้ามสรุปว่าตัวใดเร็วกว่าโดยไม่มี Same-Workload Evidence

## 10. Independent Review

หาก Codex ยังไม่สามารถใช้ Model ที่ Config ไว้ได้ ให้ตรวจสอบวิธีใช้ Default Model ที่ Account รองรับ โดยไม่แก้ Config ของผู้ใช้โดยพลการ

หากยังเรียก Independent Review ไม่สำเร็จ ให้รายงาน REVIEW PENDING และทำ Inline Review แยกต่างหาก

อย่าให้ปัญหา Codex Block การทดลอง Tauri

## 11. Decision Gate

หลังได้ผลทดสอบ ให้เสนอ Recommendation พร้อม Trade-offs

ให้ความสำคัญกับ:

1. Real-world Performance
2. Memory Efficiency
3. SQL Editor Quality
4. Data Grid Scalability
5. UX and Accessibility
6. Development and Maintenance Cost
7. Cross-platform Reliability

**ADR-0001 ต้องคง Pending จนกว่ามีหลักฐานเพียงพอ**

หากยังไม่มี Windows Test ให้ระบุ Platform Validation Pending

ADR-0002 Oracle ต้องคง Proposed เพราะยังไม่มี Live Oracle Test

## 12. Deliverables & Commit

- Tauri Prototype ที่ Build และ Run ได้จริง
- Automated Tests
- Measured Benchmark
- Complete Comparison Report
- Recommendation พร้อมหลักฐาน
- Documented Limitations
- Local Git Commit

ห้าม Push ไปยัง GitHub อัตโนมัติ

**เริ่ม T-004a ได้ทันที และอย่าหยุดเพียงการสร้าง UI Mockup ต้องสร้างตัวทดลองที่ใช้งานและวัดประสิทธิภาพได้จริง**