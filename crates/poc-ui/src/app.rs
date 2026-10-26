use crate::bench::{Bench, FrameRow, Scenario};
use crate::jsonlog::json_string;
use crate::model::editor::{line_col, statement_at};
use crate::model::explorer::{Explorer, NodeKind};
use crate::model::focus::{FocusModel, Pane};
use crate::model::grid::{
    Cell, Nav, RowSource, Selection, SyntheticSource, copy_text, scroll_to_reveal, visible_cells,
    visible_range,
};
use crate::model::highlight::{Kind, tokenize};
use crate::model::layout::Split;
use crate::model::metrics::Samples;
use crate::model::tabs::Tabs;
use crate::shortcuts;
use eframe::egui::{self, Color32, FontId, Key, Modifiers, RichText, Sense, Stroke, TextFormat};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

const ROW_H: f32 = 22.0;
const COL_W: f32 = 150.0;
const WIDE_COL_W: f32 = 320.0;
const HEADER_H: f32 = 26.0;
const OVERSCAN: usize = 3;
const MOCK_LATENCY_MS: u64 = 400;

/// Last-laid-out widget rects in egui points; logged so an external injector can aim at them.
#[derive(Clone, Copy)]
struct Rects {
    editor: egui::Rect,
    grid: egui::Rect,
    splitter: egui::Rect,
    tab0: egui::Rect,
    plus: egui::Rect,
    explorer_row1: egui::Rect,
}

impl Default for Rects {
    fn default() -> Self {
        let z = egui::Rect::ZERO;
        Rects {
            editor: z,
            grid: z,
            splitter: z,
            tab0: z,
            plus: z,
            explorer_row1: z,
        }
    }
}

enum Run {
    Idle,
    Running {
        started: Instant,
        cancel: Arc<AtomicBool>,
        rx: mpsc::Receiver<RunResult>,
    },
    Done {
        ms: u128,
        rows: usize,
    },
    Cancelled,
}

enum RunResult {
    Finished(usize),
    Cancelled,
}

pub struct Options {
    pub rows: usize,
    pub cols: usize,
    pub dark: bool,
    pub bench: Option<Scenario>,
    pub out: Option<String>,
    pub screenshot: Option<String>,
    /// Programmatic resize: send a resize command every N frames (default 1).
    pub resize_every: usize,
    /// Programmatic resize: number of frames the scenario runs after warm-up.
    pub resize_frames: usize,
    /// External scenario: wall-clock seconds to record.
    pub seconds: f64,
    /// Force-enable AccessKit tree building (diagnostic; normally activated by assistive tech).
    pub accesskit: bool,
    /// Append state-change lines (with UI rects) for external E2E input tests.
    pub event_log: Option<String>,
    pub started: Instant,
}

pub struct PocApp {
    opts: Options,
    tabs: Tabs,
    explorer: Explorer,
    src: SyntheticSource,
    sel: Selection,
    focus: FocusModel,
    editor_split: Split,
    scroll_y: f32,
    pending_scroll: Option<f32>,
    cursor_byte: usize,
    run: Run,
    cpu: Samples,
    interval: Samples,
    last_ui_start: Option<Instant>,
    last_cells: usize,
    last_range: (usize, usize),
    rss_mib: Arc<std::sync::atomic::AtomicU64>,
    last_rss_poll: Instant,
    bench: Option<Bench>,
    frame_no: usize,
    screenshot_sent: bool,
    dark: bool,
    want_editor_focus: bool,
    editor_id: Option<egui::Id>,
    rects: Rects,
    event_log: Option<std::fs::File>,
    last_snapshot: String,
    copy_requested: bool,
}

impl PocApp {
    pub fn new(cc: &eframe::CreationContext<'_>, opts: Options) -> Self {
        if opts.accesskit {
            cc.egui_ctx.enable_accesskit();
        }
        install_fonts(&cc.egui_ctx);
        apply_style(&cc.egui_ctx, opts.dark);
        let bench = opts.bench.map(|s| Bench::new(s, opts.started));
        let opts_event_log = opts
            .event_log
            .as_ref()
            .and_then(|p| std::fs::File::create(p).ok());
        let mut tabs = Tabs::new();
        tabs.active_mut().text = "SELECT o.id, o.status, o.total, c.name AS ลูกค้า\nFROM app.orders o\nJOIN app.customers c ON c.id = o.customer_id\nWHERE o.id > :min_id -- mock only; no database connection\nORDER BY o.id;\n\nSELECT COUNT(*) FROM app.orders;".into();
        Self {
            src: SyntheticSource::new(opts.rows, opts.cols),
            dark: opts.dark,
            opts,
            tabs,
            explorer: Explorer::mock(),
            sel: Selection::default(),
            focus: FocusModel::new(Pane::Editor),
            editor_split: Split::new(0.28),
            scroll_y: 0.0,
            pending_scroll: None,
            cursor_byte: 0,
            run: Run::Idle,
            cpu: Samples::new(240),
            interval: Samples::new(240),
            last_ui_start: None,
            last_cells: 0,
            last_range: (0, 0),
            rss_mib: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            last_rss_poll: Instant::now() - Duration::from_secs(10),
            bench,
            frame_no: 0,
            screenshot_sent: false,
            want_editor_focus: true,
            editor_id: None,
            rects: Rects::default(),
            event_log: opts_event_log,
            last_snapshot: String::new(),
            copy_requested: false,
        }
    }

    fn page_rows(&self, viewport_h: f32) -> usize {
        ((viewport_h / ROW_H) as usize).saturating_sub(1).max(1)
    }

    fn col_width(c: usize) -> f32 {
        if c == 4 { WIDE_COL_W } else { COL_W }
    }

    fn start_run(&mut self, ctx: &egui::Context) {
        if matches!(self.run, Run::Running { .. }) {
            return;
        }
        let text = self.tabs.active_mut().text.clone();
        let r = statement_at(&text, self.cursor_byte.min(text.len()));
        let stmt = text[r].trim();
        if stmt.is_empty() {
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        let (c2, ctx2, rows) = (cancel.clone(), ctx.clone(), self.opts.rows);
        // Simulated database latency on a worker thread: UI thread never waits.
        std::thread::spawn(move || {
            let end = Instant::now() + Duration::from_millis(MOCK_LATENCY_MS);
            while Instant::now() < end {
                if c2.load(Ordering::Relaxed) {
                    let _ = tx.send(RunResult::Cancelled);
                    ctx2.request_repaint();
                    return;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            let _ = tx.send(RunResult::Finished(rows));
            ctx2.request_repaint();
        });
        self.run = Run::Running {
            started: Instant::now(),
            cancel,
            rx,
        };
    }

    fn poll_run(&mut self) {
        if let Run::Running { started, rx, .. } = &self.run
            && let Ok(res) = rx.try_recv()
        {
            let ms = started.elapsed().as_millis();
            self.run = match res {
                RunResult::Finished(rows) => {
                    self.sel = Selection::default();
                    self.scroll_y = 0.0;
                    self.pending_scroll = Some(0.0);
                    Run::Done { ms, rows }
                }
                RunResult::Cancelled => Run::Cancelled,
            };
        }
    }

    fn handle_global_keys(&mut self, ctx: &egui::Context) {
        let sc = shortcuts::read(ctx);
        self.copy_requested = sc.copy;
        if sc.run {
            self.start_run(ctx);
        }
        if sc.new_tab {
            self.tabs.open("");
            self.focus.request(Pane::Editor);
            self.want_editor_focus = true;
        }
        if sc.close_tab {
            let i = self.tabs.active;
            self.tabs.close(i);
        }
        if sc.next_tab {
            self.tabs.next();
        }
        if sc.prev_tab {
            self.tabs.prev();
        }
        if sc.toggle_theme {
            self.dark = !self.dark;
            apply_style(ctx, self.dark);
        }
        if sc.cycle_focus {
            self.focus.request(self.focus.next_pane());
            if self.focus.current == Pane::Editor {
                self.want_editor_focus = true;
            }
        }
        if sc.cancel
            && let Run::Running { cancel, .. } = &self.run
        {
            cancel.store(true, Ordering::Relaxed);
        }
        if sc.focus_explorer {
            self.focus.request(Pane::Explorer);
        }
        if sc.focus_editor {
            self.focus.request(Pane::Editor);
            self.want_editor_focus = true;
        }
        if sc.focus_grid {
            self.focus.request(Pane::Grid);
        }
    }

    fn nav_keys(ctx: &egui::Context) -> Vec<Nav> {
        let mut out = Vec::new();
        ctx.input_mut(|i| {
            let table = [
                (Modifiers::NONE, Key::ArrowUp, Nav::Up),
                (Modifiers::NONE, Key::ArrowDown, Nav::Down),
                (Modifiers::NONE, Key::ArrowLeft, Nav::Left),
                (Modifiers::NONE, Key::ArrowRight, Nav::Right),
                (Modifiers::NONE, Key::PageUp, Nav::PageUp),
                (Modifiers::NONE, Key::PageDown, Nav::PageDown),
                (Modifiers::NONE, Key::Home, Nav::Home),
                (Modifiers::NONE, Key::End, Nav::End),
                (Modifiers::COMMAND, Key::ArrowUp, Nav::CtrlHome),
                (Modifiers::COMMAND, Key::ArrowDown, Nav::CtrlEnd),
            ];
            for (m, k, n) in table {
                while i.consume_key(m, k) {
                    out.push(n);
                }
            }
        });
        out
    }

    fn apply_grid_nav(&mut self, nav: Nav, viewport_h: f32) {
        let (rows, cols) = (self.src.row_count(), self.src.col_count());
        let page = self.page_rows(viewport_h);
        self.sel.apply(nav, rows, cols, page);
        self.pending_scroll = Some(scroll_to_reveal(
            self.sel.row,
            self.scroll_y,
            viewport_h - HEADER_H,
            ROW_H,
        ));
    }

    fn rss_poll(&mut self) {
        if self.last_rss_poll.elapsed() < Duration::from_secs(2) {
            return;
        }
        self.last_rss_poll = Instant::now();
        let slot = self.rss_mib.clone();
        std::thread::spawn(move || {
            if let Ok(o) = std::process::Command::new("ps")
                .args(["-o", "rss=", "-p", &std::process::id().to_string()])
                .output()
                && let Ok(kb) = String::from_utf8_lossy(&o.stdout).trim().parse::<u64>()
            {
                slot.store(kb / 1024, Ordering::Relaxed);
            }
        });
    }
}

impl eframe::App for PocApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let t0 = Instant::now();
        let ctx = ui.ctx().clone();
        self.frame_no += 1;
        if let Some(prev) = self.last_ui_start {
            self.interval
                .push(t0.duration_since(prev).as_secs_f32() * 1000.0);
        }
        let interval_ms = self
            .last_ui_start
            .map_or(0.0, |p| t0.duration_since(p).as_secs_f64() * 1000.0);
        self.last_ui_start = Some(t0);
        if let Some(b) = &mut self.bench
            && b.first_ui_ms.is_none()
        {
            b.first_ui_ms = Some(b.started.elapsed().as_secs_f64() * 1000.0);
        }

        self.poll_run();
        self.handle_global_keys(&ctx);
        self.rss_poll();
        self.bench_pre_frame(&ctx);

        let mut grid_viewport_h = 400.0;
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("DB Client · T-003 POC").strong());
                ui.separator();
                ui.label("Connection: Oracle DEV (mock)");
                ui.label(
                    RichText::new(" READ-ONLY ")
                        .background_color(Color32::from_rgb(40, 110, 70))
                        .color(Color32::WHITE),
                );
                ui.separator();
                let running = matches!(self.run, Run::Running { .. });
                if ui
                    .add_enabled(!running, egui::Button::new("▶ Run  ⌘↵"))
                    .clicked()
                {
                    self.start_run(&ctx);
                }
                if ui
                    .add_enabled(running, egui::Button::new("■ Cancel  Esc"))
                    .clicked()
                    && let Run::Running { cancel, .. } = &self.run
                {
                    cancel.store(true, Ordering::Relaxed);
                }
                ui.separator();
                if ui
                    .button(if self.dark { "☀ Light" } else { "🌙 Dark" })
                    .on_hover_text("⌘⇧L")
                    .clicked()
                {
                    self.dark = !self.dark;
                    apply_style(&ctx, self.dark);
                }
            });
        });

        egui::Panel::bottom("status").show(ui, |ui| {
            ui.horizontal(|ui| {
                let run_txt = match &self.run {
                    Run::Idle => "idle".to_string(),
                    Run::Running { started, .. } => format!("running… {} ms (cancel: Esc)", started.elapsed().as_millis()),
                    Run::Done { ms, rows, .. } => format!("mock query done in {ms} ms · {rows} rows (simulated {MOCK_LATENCY_MS} ms latency)"),
                    Run::Cancelled => "cancelled (mock; server state N/A)".to_string(),
                };
                ui.label(run_txt);
                ui.separator();
                let (l, c) = line_col(&self.tabs.active_mut().text, self.cursor_byte);
                ui.label(format!("Ln {l}, Col {c}"));
                ui.separator();
                ui.label(format!("rows {} · rendered rows {}..{} · cells/frame {}", self.src.row_count(), self.last_range.0, self.last_range.1, self.last_cells));
                ui.separator();
                let p = |s: &Samples, q| s.percentile(q).map_or("–".into(), |v| format!("{v:.2}"));
                ui.label(format!("ui() cpu ms p50 {} p95 {}", p(&self.cpu, 50.0), p(&self.cpu, 95.0)));
                ui.label(format!("frame Δ ms p50 {} p95 {}", p(&self.interval, 50.0), p(&self.interval, 95.0)));
                ui.separator();
                ui.label(format!("RSS {} MiB", self.rss_mib.load(Ordering::Relaxed)));
                ui.separator();
                ui.label(format!("focus: {:?} (F6)", self.focus.current));
            });
        });

        egui::Panel::left("explorer")
            .resizable(true)
            .default_size(230.0)
            .min_size(150.0)
            .show(ui, |ui| {
                ui.heading("Explorer");
                ui.small("mock data · ←→↑↓ · Alt+1");
                ui.separator();
                if self.focus.current == Pane::Explorer {
                    for n in Self::nav_keys(&ctx) {
                        self.explorer.key(n);
                    }
                }
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        let vis = self.explorer.visible();
                        for (row, &i) in vis.iter().enumerate() {
                            let depth = self.explorer.depth(i);
                            let n = &self.explorer.nodes[i];
                            let icon = match n.kind {
                                NodeKind::Connection => "◉",
                                NodeKind::Schema => "▣",
                                NodeKind::Group => {
                                    if n.expanded {
                                        "▾"
                                    } else {
                                        "▸"
                                    }
                                }
                                NodeKind::Table => "▤",
                                NodeKind::Column => "·",
                            };
                            let selected = row == self.explorer.selected;
                            let text = format!("{}{} {}", "    ".repeat(depth), icon, n.label);
                            let resp = ui.selectable_label(
                                selected && self.focus.current == Pane::Explorer,
                                text,
                            );
                            if row == 1 {
                                self.rects.explorer_row1 = resp.rect;
                            }
                            if resp.clicked() {
                                self.focus.request(Pane::Explorer);
                                self.explorer.selected = row;
                                let has_kids = !self.explorer.nodes[i].children.is_empty();
                                if has_kids {
                                    self.explorer.nodes[i].expanded =
                                        !self.explorer.nodes[i].expanded;
                                }
                            }
                            if selected && self.focus.current == Pane::Explorer {
                                resp.scroll_to_me(None);
                            }
                        }
                    });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            self.tab_bar(ui);
            let total_h = ui.available_height();
            let handle_h = 6.0;
            let editor_h = self.editor_split.pos(total_h, 90.0, 140.0).max(0.0);
            self.editor(ui, editor_h);
            self.splitter(ui, total_h, handle_h);
            grid_viewport_h = ui.available_height();
            self.grid(ui);
        });
        let _ = grid_viewport_h;

        // Keys must reach only the focused pane: release the text caret when focus moved away.
        if self.focus.end_frame()
            && let Some(id) = self.editor_id
        {
            ctx.memory_mut(|m| {
                if m.has_focus(id) {
                    m.surrender_focus(id);
                }
            });
        }

        let cpu = t0.elapsed().as_secs_f32() * 1000.0;
        self.cpu.push(cpu);
        self.bench_post_frame(&ctx, cpu as f64, interval_ms);
        self.log_events(&ctx);
        self.maybe_screenshot(&ctx);
    }
}

impl PocApp {
    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut close: Option<usize> = None;
            let mut select: Option<usize> = None;
            for (i, t) in self.tabs.tabs.iter().enumerate() {
                let active = i == self.tabs.active;
                let tab_resp = ui.selectable_label(active, &t.title);
                if i == 0 {
                    self.rects.tab0 = tab_resp.rect;
                }
                if tab_resp.clicked() {
                    select = Some(i);
                }
                if ui
                    .small_button("×")
                    .on_hover_text("Close tab (⌘W)")
                    .clicked()
                {
                    close = Some(i);
                }
                ui.add_space(6.0);
            }
            let plus = ui.button("+").on_hover_text("New tab (⌘T)");
            self.rects.plus = plus.rect;
            if plus.clicked() {
                self.tabs.open("");
                self.want_editor_focus = true;
            }
            if let Some(i) = select {
                self.tabs.active = i;
            }
            if let Some(i) = close {
                self.tabs.close(i);
            }
        });
    }

    fn editor(&mut self, ui: &mut egui::Ui, height: f32) {
        let dark = self.dark;
        let mut layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap: f32| {
            let mut job = egui::text::LayoutJob::default();
            let s = text.as_str();
            let font = FontId::monospace(14.0);
            for t in tokenize(s) {
                job.append(
                    &s[t.range.clone()],
                    0.0,
                    TextFormat {
                        font_id: font.clone(),
                        color: token_color(t.kind, dark),
                        ..Default::default()
                    },
                );
            }
            job.wrap.max_width = wrap;
            ui.fonts_mut(|f| f.layout_job(job))
        };
        let want_focus = std::mem::take(&mut self.want_editor_focus);
        let out = egui::ScrollArea::vertical()
            .id_salt("editor_scroll")
            .min_scrolled_height(height)
            .max_height(height)
            .auto_shrink(false)
            .show(ui, |ui| {
                let tab = self.tabs.active_mut();
                egui::TextEdit::multiline(&mut tab.text)
                    .id_salt(("editor", tab.id))
                    .font(FontId::monospace(14.0))
                    .desired_width(f32::INFINITY)
                    .desired_rows(8)
                    .layouter(&mut layouter)
                    .show(ui)
            });
        let te = out.inner;
        self.rects.editor = te.response.rect;
        if want_focus {
            te.response.request_focus();
        }
        self.focus
            .observe_editor_text_focus(te.response.has_focus());
        self.editor_id = Some(te.response.id);
        if let Some(r) = te.cursor_range {
            let text = self.tabs.active_mut().text.clone();
            let idx = r.primary.index.0;
            self.cursor_byte = text.char_indices().nth(idx).map_or(text.len(), |(b, _)| b);
        }
    }

    fn splitter(&mut self, ui: &mut egui::Ui, total_h: f32, handle_h: f32) {
        let (rect, resp) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), handle_h), Sense::drag());
        self.rects.splitter = rect;
        let color = if resp.hovered() || resp.dragged() {
            ui.visuals().widgets.active.bg_fill
        } else {
            ui.visuals().widgets.noninteractive.bg_stroke.color
        };
        ui.painter()
            .rect_filled(rect.shrink2(egui::vec2(0.0, 2.0)), 1.0, color);
        if resp.hovered() || resp.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
        }
        if resp.dragged() {
            let top = ui.min_rect().top();
            let _ = top;
            let editor_h = self.editor_split.pos(total_h, 90.0, 140.0) + resp.drag_delta().y;
            self.editor_split.drag_to(editor_h, total_h, 90.0, 140.0);
        }
    }

    fn grid(&mut self, ui: &mut egui::Ui) {
        let rows = self.src.row_count();
        let cols = self.src.col_count();
        let widths: Vec<f32> = (0..cols).map(Self::col_width).collect();
        let total_w: f32 = widths.iter().sum();
        let total_h = HEADER_H + rows as f32 * ROW_H;
        let avail_h = ui.available_height();
        self.rects.grid = ui.available_rect_before_wrap();

        if self.focus.current == Pane::Grid {
            for n in Self::nav_keys(&ui.ctx().clone()) {
                self.apply_grid_nav(n, avail_h);
            }
            if self.copy_requested {
                let (r, c) = (self.sel.row.min(rows.saturating_sub(1)), self.sel.col);
                ui.ctx().copy_text(copy_text(&self.src, r, c));
            }
        }

        let mut area = egui::ScrollArea::both()
            .id_salt("grid_scroll")
            .auto_shrink(false);
        if let Some(y) = self.pending_scroll.take() {
            area = area.vertical_scroll_offset(y);
        }
        let out = area.show_viewport(ui, |ui, viewport| {
            ui.set_min_size(egui::vec2(total_w, total_h));
            let origin = ui.min_rect().min;
            let body_h = (viewport.height() - HEADER_H).max(0.0);
            let range = visible_range(viewport.min.y, body_h, ROW_H, rows, OVERSCAN);
            let cells = visible_cells(&self.src, range.clone());
            self.last_cells = cells.len() * cols;
            self.last_range = (range.start, range.end);
            let painter = ui.painter_at(egui::Rect::from_min_size(
                origin + egui::vec2(viewport.min.x, viewport.min.y),
                viewport.size(),
            ));
            let vis = ui.visuals().clone();
            let font = FontId::proportional(13.0);
            let mono = FontId::monospace(13.0);
            let mut x_off = Vec::with_capacity(cols + 1);
            let mut acc = 0.0;
            for w in &widths {
                x_off.push(acc);
                acc += w;
            }
            for (ri, row_cells) in cells.iter().enumerate() {
                let r = range.start + ri;
                let y = origin.y + HEADER_H + r as f32 * ROW_H;
                // Cull horizontally too: draw only columns intersecting the viewport.
                if r % 2 == 1 {
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(origin.x + viewport.min.x, y),
                            egui::vec2(viewport.width(), ROW_H),
                        ),
                        0.0,
                        vis.faint_bg_color,
                    );
                }
                for (c, cell) in row_cells.iter().enumerate() {
                    let x = origin.x + x_off[c];
                    if x + widths[c] < origin.x + viewport.min.x || x > origin.x + viewport.max.x {
                        continue;
                    }
                    let rect =
                        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(widths[c], ROW_H));
                    let selected = r == self.sel.row && c == self.sel.col;
                    if selected {
                        painter.rect_filled(rect, 0.0, vis.selection.bg_fill.gamma_multiply(0.5));
                        let ring = if self.focus.current == Pane::Grid {
                            vis.selection.stroke.color
                        } else {
                            vis.weak_text_color()
                        };
                        painter.rect_stroke(
                            rect.shrink(1.0),
                            0.0,
                            Stroke::new(2.0, ring),
                            egui::StrokeKind::Inside,
                        );
                    }
                    let clip = painter.with_clip_rect(
                        rect.shrink2(egui::vec2(2.0, 0.0))
                            .intersect(painter.clip_rect()),
                    );
                    let pad = egui::pos2(x + 6.0, y + ROW_H / 2.0);
                    match cell {
                        Cell::Null => {
                            clip.text(
                                pad,
                                egui::Align2::LEFT_CENTER,
                                "<NULL>",
                                font.clone(),
                                vis.weak_text_color(),
                            );
                        }
                        Cell::Number(n) => {
                            let a = egui::pos2(x + widths[c] - 6.0, y + ROW_H / 2.0);
                            clip.text(
                                a,
                                egui::Align2::RIGHT_CENTER,
                                n,
                                mono.clone(),
                                vis.text_color(),
                            );
                        }
                        Cell::Text(t) => {
                            let shown: String = t.chars().take(90).collect();
                            clip.text(
                                pad,
                                egui::Align2::LEFT_CENTER,
                                shown,
                                font.clone(),
                                vis.text_color(),
                            );
                        }
                    }
                }
            }
            // Sticky header drawn last at viewport top.
            let hy = origin.y + viewport.min.y;
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(origin.x + viewport.min.x, hy),
                    egui::vec2(viewport.width(), HEADER_H),
                ),
                0.0,
                vis.extreme_bg_color,
            );
            for c in 0..cols {
                let x = origin.x + x_off[c];
                if x + widths[c] < origin.x + viewport.min.x || x > origin.x + viewport.max.x {
                    continue;
                }
                let name = self.src.column_name(c);
                painter.text(
                    egui::pos2(x + 6.0, hy + HEADER_H / 2.0),
                    egui::Align2::LEFT_CENTER,
                    name,
                    FontId::proportional(13.0),
                    vis.strong_text_color(),
                );
                painter.line_segment(
                    [
                        egui::pos2(x + widths[c], hy),
                        egui::pos2(x + widths[c], hy + HEADER_H),
                    ],
                    vis.widgets.noninteractive.bg_stroke,
                );
            }
            // Click to select.
            let resp = ui.interact(
                egui::Rect::from_min_size(origin + viewport.min.to_vec2(), viewport.size()),
                ui.scope_id().with("grid_click"),
                Sense::click(),
            );
            if resp.clicked() {
                self.focus.request(Pane::Grid);
                if let Some(p) = resp.interact_pointer_pos() {
                    let ly = p.y - origin.y - HEADER_H;
                    let lx = p.x - origin.x;
                    if ly >= 0.0 {
                        let row = (ly / ROW_H) as usize;
                        let col = x_off.iter().rposition(|&o| o <= lx).unwrap_or(0);
                        if row < rows {
                            self.sel = Selection {
                                row,
                                col: col.min(cols.saturating_sub(1)),
                            };
                        }
                    }
                }
            }
        });
        self.scroll_y = out.state.clamped_offset().y;
    }

    /// One line whenever observable UI state changes; used by external E2E scripts.
    fn log_events(&mut self, ctx: &egui::Context) {
        use std::io::Write as _;
        if self.event_log.is_none() {
            return;
        }
        let inner = ctx
            .input(|i| i.viewport().inner_rect)
            .unwrap_or(egui::Rect::ZERO);
        let ppp = ctx.pixels_per_point();
        let (tab_count, active, tail, chars) = {
            let t = &self.tabs.tabs[self.tabs.active];
            let tail: String = t
                .text
                .chars()
                .rev()
                .take(40)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            (
                self.tabs.tabs.len(),
                self.tabs.active,
                tail,
                t.text.chars().count(),
            )
        };
        let r = |x: egui::Rect| {
            format!(
                "[{:.1},{:.1},{:.1},{:.1}]",
                x.min.x, x.min.y, x.max.x, x.max.y
            )
        };
        let editor_focus = self
            .editor_id
            .is_some_and(|id| ctx.memory(|m| m.has_focus(id)));
        let snap = format!(
            "\"sel\":[{},{}],\"focus\":\"{:?}\",\"tab\":{},\"tabs\":{},\"chars\":{},\"tail\":{},\"split\":{:.3},\"scroll_row\":{},\"exp_sel\":{},\"dark\":{},\"editor_has_focus\":{},\"run\":\"{}\"",
            self.sel.row,
            self.sel.col,
            self.focus.current,
            active,
            tab_count,
            chars,
            json_string(&tail),
            self.editor_split.ratio,
            (self.scroll_y / ROW_H) as usize,
            self.explorer.selected,
            self.dark,
            editor_focus,
            match self.run {
                Run::Idle => "idle",
                Run::Running { .. } => "running",
                Run::Done { .. } => "done",
                Run::Cancelled => "cancelled",
            }
        );
        let layout = format!(
            "\"ppp\":{ppp},\"inner\":{},\"editor\":{},\"grid\":{},\"splitter\":{},\"tab0\":{},\"plus\":{},\"exp_row1\":{}",
            r(inner),
            r(self.rects.editor),
            r(self.rects.grid),
            r(self.rects.splitter),
            r(self.rects.tab0),
            r(self.rects.plus),
            r(self.rects.explorer_row1)
        );
        let key = format!("{snap}|{layout}");
        if key == self.last_snapshot {
            return;
        }
        self.last_snapshot = key;
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        if let Some(f) = &mut self.event_log {
            let _ = writeln!(
                f,
                "{{\"ts_ns\":{ts},\"frame\":{},{snap},{layout}}}",
                self.frame_no
            );
            let _ = f.flush();
        }
    }

    // ---- benchmark scripting -------------------------------------------------------------
    fn bench_pre_frame(&mut self, ctx: &egui::Context) {
        let Some(b) = &self.bench else { return };
        let scenario = b.scenario;
        let frame = b.frames;
        let rows = self.src.row_count();
        let view_h = 700.0;
        match scenario {
            Scenario::Scroll => {
                ctx.request_repaint();
                if frame >= 60 {
                    let f = frame - 60;
                    let max = (rows as f32 * ROW_H - view_h).max(0.0);
                    let y = if f < 300 {
                        f as f32 * 3.0 * ROW_H
                    } else {
                        ((f - 300) as f32 / 300.0) * max
                    };
                    self.pending_scroll = Some(y.min(max));
                }
            }
            Scenario::Nav => {
                ctx.request_repaint();
                if frame >= 60 {
                    self.focus.request(Pane::Grid);
                    let f = frame - 60;
                    let nav = if f % 200 < 150 {
                        Nav::Down
                    } else if f % 200 < 175 {
                        Nav::Right
                    } else {
                        Nav::PageDown
                    };
                    self.apply_grid_nav(nav, view_h);
                }
            }
            Scenario::Resize => {
                ctx.request_repaint();
                if frame >= 30 && (frame - 30).is_multiple_of(self.opts.resize_every.max(1)) {
                    let f = (frame - 30) as f32;
                    let w = 1100.0 + 300.0 * (f / 12.0).sin();
                    let h = 760.0 + 140.0 * (f / 17.0).cos();
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(w, h)));
                }
            }
            Scenario::External => ctx.request_repaint(),
            Scenario::Startup | Scenario::Idle => {}
        }
    }

    fn bench_post_frame(&mut self, ctx: &egui::Context, cpu_ms: f64, interval_ms: f64) {
        let rect = ctx.content_rect();
        let (cells, vis) = (self.last_cells, self.last_range.1 - self.last_range.0);
        let Some(b) = &mut self.bench else { return };
        let f = b.frames;
        b.frames += 1;
        let phase = match b.scenario {
            Scenario::Scroll if f >= 60 && f - 60 < 300 => "slow",
            Scenario::Scroll if f >= 60 => "sweep",
            Scenario::Nav if f >= 60 => "nav",
            Scenario::Resize if f >= 30 => "resize",
            Scenario::External if f >= 30 => "external",
            _ => "warmup",
        };
        b.rows.push(FrameRow {
            phase,
            frame: f,
            cpu_ms,
            interval_ms,
            cells,
            vis_rows: vis,
            win_w: rect.width(),
            win_h: rect.height(),
        });
        let done = match b.scenario {
            Scenario::Startup => {
                if f == 5 && b.ready_ms.is_none() {
                    b.ready_ms = Some(b.started.elapsed().as_secs_f64() * 1000.0);
                    println!(
                        "READY first_ui_ms={:.2} ready_ms={:.2}",
                        b.first_ui_ms.unwrap_or(-1.0),
                        b.ready_ms.unwrap()
                    );
                }
                f >= 6
            }
            Scenario::Scroll => f >= 60 + 600,
            Scenario::Nav => f >= 60 + 400,
            Scenario::Resize => f >= 30 + self.opts.resize_frames,
            Scenario::External => {
                if f == 5 && b.ready_ms.is_none() {
                    b.ready_ms = Some(b.started.elapsed().as_secs_f64() * 1000.0);
                    println!(
                        "READY first_ui_ms={:.2} ready_ms={:.2}",
                        b.first_ui_ms.unwrap_or(-1.0),
                        b.ready_ms.unwrap()
                    );
                }
                b.started.elapsed().as_secs_f64() > self.opts.seconds
            }
            Scenario::Idle => {
                if f == 5 && b.ready_ms.is_none() {
                    b.ready_ms = Some(b.started.elapsed().as_secs_f64() * 1000.0);
                    println!(
                        "READY first_ui_ms={:.2} ready_ms={:.2}",
                        b.first_ui_ms.unwrap_or(-1.0),
                        b.ready_ms.unwrap()
                    );
                }
                false // harness kills the process after sampling
            }
        };
        if done {
            if let Some(path) = &self.opts.out {
                let _ = std::fs::write(path, b.csv());
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn maybe_screenshot(&mut self, ctx: &egui::Context) {
        let Some(path) = self.opts.screenshot.clone() else {
            return;
        };
        if self.frame_no == 20 && !self.screenshot_sent {
            self.screenshot_sent = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(
                egui::ScreenshotCallback::new(move |img| {
                    let _ = write_png(&path, &img);
                    std::process::exit(0);
                }),
            ));
        }
        if self.frame_no < 40 {
            ctx.request_repaint();
        }
    }
}

fn write_png(path: &str, img: &egui::ColorImage) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(
        std::io::BufWriter::new(file),
        img.width() as u32,
        img.height() as u32,
    );
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(std::io::Error::other)?;
    let data: Vec<u8> = img.pixels.iter().flat_map(|p| p.to_array()).collect();
    w.write_image_data(&data).map_err(std::io::Error::other)
}

fn token_color(k: Kind, dark: bool) -> Color32 {
    match (k, dark) {
        (Kind::Keyword, true) => Color32::from_rgb(120, 170, 255),
        (Kind::Keyword, false) => Color32::from_rgb(0, 80, 200),
        (Kind::String, true) => Color32::from_rgb(160, 210, 120),
        (Kind::String, false) => Color32::from_rgb(20, 110, 40),
        (Kind::Comment, true) => Color32::from_rgb(130, 140, 150),
        (Kind::Comment, false) => Color32::from_rgb(100, 110, 120),
        (Kind::Number, true) => Color32::from_rgb(240, 170, 100),
        (Kind::Number, false) => Color32::from_rgb(170, 80, 0),
        (Kind::Bind, true) => Color32::from_rgb(230, 130, 200),
        (Kind::Bind, false) => Color32::from_rgb(160, 30, 120),
        (Kind::QuotedIdent, true) => Color32::from_rgb(120, 210, 210),
        (Kind::QuotedIdent, false) => Color32::from_rgb(0, 110, 110),
        (_, true) => Color32::from_gray(220),
        (_, false) => Color32::from_gray(30),
    }
}

fn apply_style(ctx: &egui::Context, dark: bool) {
    ctx.set_theme(if dark {
        egui::ThemePreference::Dark
    } else {
        egui::ThemePreference::Light
    });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(8.0, 6.0);
        s.spacing.button_padding = egui::vec2(10.0, 5.0);
        s.spacing.interact_size.y = 28.0;
        s.visuals.window_corner_radius = 6.0.into();
        s.visuals.widgets.noninteractive.corner_radius = 4.0.into();
        s.visuals.widgets.inactive.corner_radius = 4.0.into();
        s.visuals.widgets.hovered.corner_radius = 4.0.into();
        s.visuals.widgets.active.corner_radius = 4.0.into();
    });
}

/// Thai glyphs come from a system font at runtime (not bundled; licensing) if present.
fn install_fonts(ctx: &egui::Context) {
    let candidates = [
        "/System/Library/Fonts/Supplemental/Ayuthaya.ttf",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
    ];
    let mut fonts = egui::FontDefinitions::default();
    for (i, path) in candidates.iter().enumerate() {
        if let Ok(bytes) = std::fs::read(path) {
            let name = format!("thai_fallback_{i}");
            fonts
                .font_data
                .insert(name.clone(), Arc::new(egui::FontData::from_owned(bytes)));
            for fam in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts.families.entry(fam).or_default().push(name.clone());
            }
            break;
        }
    }
    ctx.set_fonts(fonts);
}
