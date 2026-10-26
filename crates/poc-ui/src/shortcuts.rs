//! Global shortcut detection, separated from `app` so it can be tested headlessly with
//! synthetic egui input. Regressions found by real-input E2E (2026-10-26):
//! * Ctrl+Shift+Tab was swallowed by the broader Ctrl+Tab matcher (previous-tab acted as next).
//! * Cmd+C never reached the grid: egui turns it into `Event::Copy`, not a key event.
use eframe::egui::{self, Event, Key, Modifiers};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Shortcuts {
    pub run: bool,
    pub new_tab: bool,
    pub close_tab: bool,
    pub next_tab: bool,
    pub prev_tab: bool,
    pub toggle_theme: bool,
    pub cycle_focus: bool,
    pub cancel: bool,
    pub copy: bool,
    pub focus_explorer: bool,
    pub focus_editor: bool,
    pub focus_grid: bool,
}

/// Consume and report this frame's global shortcuts. Most specific modifier sets first.
pub fn read(ctx: &egui::Context) -> Shortcuts {
    let cmd = Modifiers::COMMAND;
    ctx.input_mut(|i| {
        // More specific modifier sets first: egui's consume_key tolerates extra Shift, so
        // plain Ctrl+Tab would otherwise also consume Ctrl+Shift+Tab.
        let prev_tab = i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Tab);
        let next_tab = i.consume_key(Modifiers::CTRL, Key::Tab);
        Shortcuts {
            run: i.consume_key(cmd, Key::Enter),
            new_tab: i.consume_key(cmd, Key::T),
            close_tab: i.consume_key(cmd, Key::W),
            next_tab,
            prev_tab,
            toggle_theme: i.consume_key(cmd | Modifiers::SHIFT, Key::L),
            cycle_focus: i.consume_key(Modifiers::NONE, Key::F6),
            cancel: i.key_pressed(Key::Escape),
            copy: i.events.iter().any(|e| matches!(e, Event::Copy)),
            focus_explorer: i.consume_key(Modifiers::ALT, Key::Num1),
            focus_editor: i.consume_key(Modifiers::ALT, Key::Num2),
            focus_grid: i.consume_key(Modifiers::ALT, Key::Num3),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_event(key: Key, modifiers: Modifiers) -> Event {
        Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn run_with(events: Vec<Event>) -> Shortcuts {
        let ctx = egui::Context::default();
        let mut out = Shortcuts::default();
        let input = egui::RawInput {
            events,
            ..Default::default()
        };
        let mut full = ctx.run_ui(input, |ui| out = read(ui.ctx()));
        full.textures_delta.clear();
        out
    }

    #[test]
    fn ctrl_tab_is_next_only() {
        let s = run_with(vec![key_event(Key::Tab, Modifiers::CTRL)]);
        assert!(s.next_tab && !s.prev_tab, "{s:?}");
    }

    #[test]
    fn ctrl_shift_tab_is_prev_only() {
        let s = run_with(vec![key_event(
            Key::Tab,
            Modifiers::CTRL | Modifiers::SHIFT,
        )]);
        assert!(s.prev_tab && !s.next_tab, "{s:?}");
    }

    #[test]
    fn copy_event_is_reported() {
        assert!(run_with(vec![Event::Copy]).copy);
    }

    #[test]
    fn cmd_enter_runs_and_cmd_t_w_manage_tabs() {
        assert!(run_with(vec![key_event(Key::Enter, Modifiers::COMMAND)]).run);
        assert!(run_with(vec![key_event(Key::T, Modifiers::COMMAND)]).new_tab);
        assert!(run_with(vec![key_event(Key::W, Modifiers::COMMAND)]).close_tab);
    }

    #[test]
    fn cmd_shift_l_toggles_theme_and_plain_l_does_not() {
        assert!(
            run_with(vec![key_event(
                Key::L,
                Modifiers::COMMAND | Modifiers::SHIFT
            )])
            .toggle_theme
        );
        assert!(!run_with(vec![key_event(Key::L, Modifiers::NONE)]).toggle_theme);
    }

    #[test]
    fn f6_alt_digits_and_escape() {
        assert!(run_with(vec![key_event(Key::F6, Modifiers::NONE)]).cycle_focus);
        assert!(run_with(vec![key_event(Key::Num1, Modifiers::ALT)]).focus_explorer);
        assert!(run_with(vec![key_event(Key::Num2, Modifiers::ALT)]).focus_editor);
        assert!(run_with(vec![key_event(Key::Num3, Modifiers::ALT)]).focus_grid);
        assert!(run_with(vec![key_event(Key::Escape, Modifiers::NONE)]).cancel);
    }

    #[test]
    fn no_events_no_shortcuts() {
        assert_eq!(run_with(vec![]), Shortcuts::default());
    }
}
