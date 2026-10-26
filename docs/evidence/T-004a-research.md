# T-004a — dependency and measurement research

Research date: 2026-10-26 (Asia/Bangkok). Scope: implementation guidance for the isolated Tauri experiment; **not benchmark results, a completed dependency audit, or a UI-stack decision**. ADR-0001 remains Pending. Research skill used with a background agent; AGENTS.md and SKILLS.md read. No application/config changes, dependency installation, database access, or commits performed by this research task.

## Registry snapshot and compatibility

Read-only commands executed: `npm view PACKAGE version license engines peerDependencies dist.unpackedSize --json`; selected `dependencies`, `peerDependenciesMeta`, and `repository` fields; crates.io API `/api/v1/crates/NAME`. Local `node --version` = **v24.10.0**, npm = **11.6.0**. These are candidate pins, not a claim the final application uses them. Its package-lock.json/Cargo.lock and actual build are authoritative.

| Package | Registry version observed | License | Engine / MSRV | Registry package size, bytes |
|---|---:|---|---|---:|
| @tauri-apps/api | 2.12.4 | Apache-2.0 OR MIT | JS API; no engine field returned | 892,209 unpacked |
| @tauri-apps/cli | 2.12.4 | Apache-2.0 OR MIT | Node >=10 metadata | 432,536 unpacked; excludes platform binary package |
| tauri | 2.12.5 | Apache-2.0 OR MIT | Rust 1.90 declared | 329,243 crate archive |
| tauri-build | 2.7.4 | Apache-2.0 OR MIT | Rust 1.90 declared | 44,842 crate archive |
| react | 19.3.1 | MIT | Node >=0.10 metadata | 178,663 unpacked |
| react-dom | 19.3.1 | MIT | React ^19.3.1 peer | 8,067,632 unpacked |
| vite | 8.3.5 | MIT | Node ^20.19.0 or >=22.12.0 | 2,383,283 unpacked |
| @vitejs/plugin-react | 6.1.2 | MIT | Same Node range; Vite ^8 peer | 46,296 unpacked |
| typescript | 7.0.2 | Apache-2.0 | Node >=16.20 metadata | 2,497,498 unpacked |
| @codemirror/state | 6.7.6 | MIT | Browser JS; no Rust MSRV | 440,367 unpacked |
| @codemirror/view | 6.43.15 | MIT | Browser JS; no Rust MSRV | 1,264,544 unpacked |
| @codemirror/lang-sql | 6.10.0 | MIT | Browser JS; no Rust MSRV | 146,145 unpacked |
| @codemirror/commands | 6.11.1 | MIT | Browser JS; no Rust MSRV | 246,640 unpacked |
| @codemirror/theme-one-dark | 6.1.3 | MIT | Browser JS; no Rust MSRV | 20,037 unpacked |
| @tanstack/react-virtual | 3.14.23 | MIT | React/React DOM 16.8–19 peer ranges | 63,728 unpacked |

Primary metadata endpoints: [Tauri API](https://registry.npmjs.org/@tauri-apps%2fapi/2.12.4), [Tauri CLI](https://registry.npmjs.org/@tauri-apps%2fcli/2.12.4), [React](https://registry.npmjs.org/react/19.3.1), [React DOM](https://registry.npmjs.org/react-dom/19.3.1), [Vite](https://registry.npmjs.org/vite/8.3.5), [Vite React plugin](https://registry.npmjs.org/@vitejs%2fplugin-react/6.1.2), [TypeScript](https://registry.npmjs.org/typescript/7.0.2), [CodeMirror state](https://registry.npmjs.org/@codemirror%2fstate/6.7.6), [view](https://registry.npmjs.org/@codemirror%2fview/6.43.15), [SQL](https://registry.npmjs.org/@codemirror%2flang-sql/6.10.0), [commands](https://registry.npmjs.org/@codemirror%2fcommands/6.11.1), [theme](https://registry.npmjs.org/@codemirror%2ftheme-one-dark/6.1.3), [TanStack React Virtual](https://registry.npmjs.org/@tanstack%2freact-virtual/3.14.23), [tauri crate](https://crates.io/api/v1/crates/tauri/2.12.5), [tauri-build crate](https://crates.io/api/v1/crates/tauri-build/2.7.4).

Package sizes above are **not shipped artifact sizes**. npm packages include source/maps/development builds; Tauri CLI pulls a platform-specific native package. The release binary/bundle and generated frontend assets must be measured separately. Declared crate MSRV does not establish the whole dependency graph's effective MSRV.

Vite plugin React's compiler-related peers (`oxc-transform-react`, `@rolldown/plugin-babel`, `babel-plugin-react-compiler`) are marked optional in registry metadata; no need to add a compiler merely to satisfy the list. TanStack React Virtual 3.14.23 resolves **virtual-core 3.19.8 exactly**. TypeScript 6.0.3 was also verified to exist as a conservative fallback if tooling has not adopted 7; choosing it needs an explicit pin, not an untracked global compiler assumption.

## API choices and implementation boundaries

- Tauri recommends Vite for a React SPA. Production `frontendDist` points at `../dist`; the release executable must serve embedded built assets. `devUrl`/Vite development mode cannot supply decision benchmark numbers. [Frontend configuration](https://v2.tauri.app/start/frontend/), [Vite integration](https://v2.tauri.app/start/frontend/vite/).
- Rust commands expose typed arguments/results via `invoke`. Heavy work belongs in asynchronous tasks or blocking workers; a synchronous command otherwise runs on the main thread. Keep the mock query delay/cancellation state independent of rendering and distinguish cancellation requested from completed. Bound IPC arguments and do not expose arbitrary SQL, shell, or file operations in this mock experiment. [Calling Rust](https://v2.tauri.app/develop/calling-rust/).
- Capabilities scope core/plugin permissions by window. Application commands registered with `invoke_handler` are allowed to application windows by default; adding `core:default` is not an authorization policy for arbitrary custom commands. Restrict navigation to bundled content, configure CSP, and grant only needed operations. CSS-in-JS styling/CodeMirror style insertion may need a deliberate style policy; do not fix a styling issue by disabling script CSP. [Capabilities](https://v2.tauri.app/security/capabilities/), [CSP](https://v2.tauri.app/security/csp/).
- CodeMirror `EditorView` renders the viewport plus margin and exposes `visibleRanges`. Preserve one `EditorState` per tab; swapping the visible state preserves selection/history without one active editor DOM per tab. `EditorView.updateListener` provides an app-state seam; call `destroy()` on unmount. SQL highlighting is an editor aid, not proof of Oracle SQL/PLSQL parse correctness. [System guide](https://codemirror.net/docs/guide/), [reference](https://codemirror.net/docs/ref/).
- TanStack Virtual combines a vertical virtualizer and a horizontal virtualizer on the same scroll element. Use fixed 22px rows, stable keys, bounded overscan, and actual visible items for DOM creation; count the rendered rows/cells during runs. Keep selection in logical row/column coordinates and explicitly scroll selected cells into view. [Introduction](https://tanstack.com/virtual/latest/docs/introduction), [Virtualizer API](https://tanstack.com/virtual/latest/docs/api/virtualizer).

## Grid scaling pitfalls

**DOM virtualization does not imply O(viewport) total memory.** The npm source tarball for [virtual-core 3.19.8](https://registry.npmjs.org/@tanstack/virtual-core/-/virtual-core-3.19.8.tgz) was inspected read-only in memory. `getMeasurements` in `package/src/index.ts` lines 1663–1708 uses single-lane lazy object materialization, but still allocates a `Float64Array(count * 2)` plus `Array(count)` keys and initializes every item. For 1M rows the numeric backing alone is 16,000,000 bytes; keys/objects/GC add overhead. Report this as a real difference against egui's arithmetic fixed-row implementation. Generating cells only on demand does not remove library metadata cost. Do not silently segment one side's workload to disguise it.

At 1M × 22px, virtual content height is **22,000,000 CSS px**. WebKit's current LayoutUnit source stores a signed integer with 6 fractional bits, implying an approximate 33,554,431px representable upper bound. This is a source-derived bound, **not a verified guarantee about every scroll/compositing path or platform WebView**. Verify actual `scrollHeight`, jump to final row, selected index, visible last row, and horizontal final column in the built WKWebView. Larger row heights, zoom, or a different engine may require segmented scrolling. [WebKit LayoutUnit source](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/LayoutUnit.h), [design explanation](https://trac.webkit.org/wiki/LayoutUnit).

## Memory attribution protocol

Tauri has a core process plus OS WebView processes; macOS uses WKWebView. WebContent runs page code and networking/storage services can live separately. Therefore main PID RSS cannot represent Tauri total memory. OS WebView libraries are dynamically supplied, so small artifact size does not mean their runtime cost is zero. [Tauri process model](https://v2.tauri.app/concept/process-model/), [WebKit architecture](https://docs.webkit.org/Deep%20Dive/Architecture/WebKit2.html).

Proposed measurement method (must be validated in the actual run):

1. Save a before-launch inventory of relevant process PIDs/start times/names. Launch only the experiment and record its main PID.
2. Find candidate WebContent, Networking, GPU, and direct descendants. Treat temporal co-appearance only as a candidate signal.
3. Require an ownership link: an OS report's `Responsible`/application coalition identity, app-specific service identity, or verified parent chain to this app. Save the exact evidence. WKWebView XPC services may have launchd as parent; PPID alone may miss them. Name matching alone can include another application's WebKit processes.
4. Poll the established attributed set during the workload; record exits/restarts/new helper evidence. Report main RSS and attributed-group RSS separately. A group with unresolved helper ownership is incomplete, not a total.
5. Sum RSS only with a shared-page double-count warning. Run `footprint -p MAIN_PID -p HELPER_PID ... -j OUTPUT.json` outside timing-critical loops when permitted. The installed macOS `footprint(1)` manual says multiple-process analysis de-duplicates multiply-mapped objects and exposes `phys_footprint`/`phys_footprint_peak`; shared wired memory has limitations. Keep this different metric separate from RSS. Do not run a wildcard `footprint WebKit` and call it app memory.

Primary tool documentation: installed `man footprint` read during research; [WebKit Memory Inspection](https://docs.webkit.org/Infrastructure/MemoryInspection.html) documents footprint categories and auxiliary physical-footprint fields. Process ownership and available permissions remain an empirical gate, **not established by this note**. Do not terminate unrelated user applications to simplify attribution.

### Read-only ownership probe follow-up

`benchmarks/t004a/ownership.py` implements a local macOS observer with two **private, source-verified, read-only APIs**, isolated from application code. This is not a portable supported OS interface:

- `responsibility_get_pid_responsible_for_pid(pid_t) -> pid_t`: signature and use verified in [Chromium's implementation](https://chromium.googlesource.com/chromium/src/+/refs/tags/134.0.6998.28/base/process/process_info_mac.mm); exported symbol verified in the installed SDK's `usr/lib/libSystem.B.tbd` and `usr/lib/system/libquarantine.tbd`. A live self query succeeded without root.
- `proc_pidinfo` coalition flavor and struct taken from **pinned Apple source**, not guessed: [proc_info_private.h](https://github.com/apple-oss-distributions/xnu/blob/f6217f891ac0bb64f3d375211650a4c1ff8ca1ea/bsd/sys/proc_info_private.h) defines flavor 20 and coalition IDs plus three reserved uint64 fields; [mach/coalition.h](https://github.com/apple-oss-distributions/xnu/blob/f6217f891ac0bb64f3d375211650a4c1ff8ca1ea/osfmk/mach/coalition.h) defines RESOURCE=0, JETSAM=1, NUM_TYPES=2. The observer requires exactly 40 returned bytes; a live self query returned 40 with errno 0.

During a direct executable launch, Tauri PID **54504** and WebKit GPU/Networking/WebContent PIDs **54505/54506/54507** all reported responsible PID **63705**, resource coalition **34399**, jetsam coalition **38688**. The observer also shared those coalition IDs: the launcher's group had been inherited. Therefore **these facts alone did not prove app-specific helper ownership**. Other running WebKit helpers had different responsible PIDs/coalitions. The classifier deliberately does not aggregate a shared launcher group.

The helper returns PIDs only for the main process, explicit `responsible_pid == main_pid`, or a direct parent chain into the attributed set. Coalition fields are corroboration only. Missing symbols, OS failure, an absent app PID, or unresolved ownership must remain visible. Five classifier tests ran red against the stub and then green; logs: `benchmarks/t004a/raw/ownership-{red,green}.log`. Script import/compile checked with `python3 -m py_compile`; no app or user configuration changed.

**LaunchServices validation succeeded in a later sustained run.** PID **57930** reported itself responsible, with resource/jetsam coalitions **41550/41551**. Six helpers independently reported responsible PID 57930 and the same coalitions: GPU **57931**, Networking **57932**, WebContent **57933**, audio SandboxHelper **57934**, SafariPlatformSupport.Helper **57938**, ThemeWidgetControlViewService **59095**. The latter three show why a WebKit-name-only sum can undercount even after the main process problem is fixed. Evidence: `benchmarks/t004a/raw/ownership-sustained-57930.json`.

At 2026-10-26 23:05:20 +07:00, `footprint` across these seven PIDs succeeded (exit 0, approximately 0.08s), producing de-duplicated **Summary Footprint 290,886,816 bytes**. The adjacent `ps` snapshot recorded main-only RSS **101,184 KiB**, summed attributed RSS **352,208 KiB**. These are a **method-validation snapshot of an unclassified workload state**, not a 60-second idle baseline or comparison result. Keep the metrics distinct; RSS includes shared mappings and the footprint measure includes different accounting categories. Raw: `footprint-sustained-57930.{json,txt}`, `rss-sustained-57930.txt` in `benchmarks/t004a/raw/`. This observer did not launch or terminate the app; the parent benchmark session controlled its lifetime.

## Timing definitions and instrumentation

| Stage | Suggested evidence | What it does not establish |
|---|---|---|
| JS synchronous handler | `performance.now()` before/after handler logic | React render/layout/GPU/display |
| App-state acceptance | OS event injection timestamp to state-change record, with documented clock alignment; or browser event timestamp to state acceptance | Physical key hardware latency; latter excludes unmeasured OS stages |
| React state-to-commit observation | `useLayoutEffect` after target state commits, correlate event ID | React CPU time; browser paint completion |
| React render CPU | Profiling-enabled production build only, separate diagnostic experiment | Equivalent workload to uninstrumented final release |
| Browser frame interval | Consecutive `requestAnimationFrame` timestamps while foreground/active | Actual display scanout, GPU present, per-frame CPU cost |
| Rust mock work | Rust monotonic timer around worker, report IPC roundtrip separately | Frontend render or end-to-end input latency |
| Input-to-display | Validated presentation/trace or external high-speed capture | Cannot be inferred from handler return or two rAF callbacks |

React `<Profiler>` is disabled by default in the ordinary production build. Enabling a profiling build adds overhead and must not silently replace the final release benchmark. [React Profiler](https://react.dev/reference/react/Profiler), [React performance tracks](https://react.dev/reference/dev-tools/react-performance-tracks).

The Event Timing specification distinguishes event creation, dispatch/processing, and the subsequent rendering update; duration is quantized. Feature-detect actual WebView support and capture it in environment metadata rather than assuming compatibility from desktop browser documentation. It still does not measure monitor scanout. [W3C Event Timing](https://www.w3.org/TR/event-timing/).

In particular, egui `ui()` CPU and React commit observations are different intervals. Comparable high-level metrics are startup under a shared readiness contract, same action→app-state acceptance, frame intervals during the same scroll schedule, CPU consumed per matched wall-clock interval, and attributed memory under a matched state. Keep internal stage metrics in separate columns. Disable continuous diagnostic rAF/telemetry polling for true idle measurements or state explicitly that it remains active on both sides.

## Maintenance, security, signing, and remaining gates

- npm/crates metadata show actively published versions. CodeMirror's GitHub repositories were archived during a hosting move; the current npm repository points to `code.haverbeke.berlin`. Do not interpret that archive alone as project abandonment. Direct browsing of the new host returned HTTP 403 in this environment; the registry repository URL and [official old-repository move notice](https://github.com/codemirror/view) were available.
- **CVE/complete transitive license audit: NOT DONE by this research task.** Metadata license strings are an initial inventory, not legal review, SBOM, provenance verification, or proof of no vulnerabilities. Run npm audit against the final lockfile and Cargo audit/deny if installed; preserve tool errors. No unreviewed audit tooling was installed here.
- Tauri's platform crates and system WebView create OS-version maintenance obligations. macOS results do not establish Windows WebView2 or Linux WebKitGTK behavior. [Process model](https://v2.tauri.app/concept/process-model/).
- A local ad-hoc signed experiment is distinct from distributable signing/notarization. Do not access credentials or modify user signing configuration. Production distribution needs the relevant certificate/notarization and dependency notice workflow. [macOS signing](https://v2.tauri.app/distribute/sign/macos/).
- Tauri's current official test documentation describes an embedded WebDriver service supporting macOS; direct `tauri-driver` remains Windows/Linux only. Installing test plugins changes the binary, so distinguish diagnostic automation builds from decision benchmarks. Existing CGEvent input plus bounded app-state logging can avoid that extra runtime for this experiment. [Tauri WebDriver](https://v2.tauri.app/develop/tests/webdriver/).
- Accessibility requires actual semantic/focus inspection and interaction tests. React/WebView does not automatically make a virtualized grid accessible. VoiceOver, real Thai IME composition, Windows/Linux, font licensing/rendering, and physical input-to-display remain **NOT TESTED** until separate evidence exists.

Recommendation at research stage: proceed with the specified Tauri/CodeMirror/TanStack stack on the experiment branch; pin the resolved graph, measure library metadata and attributed WebView memory honestly, and keep ADR-0001 Pending until same-workload and platform gates are satisfied.
