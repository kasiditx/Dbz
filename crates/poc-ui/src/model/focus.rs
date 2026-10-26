//! Pane focus with the per-frame ordering that the immediate-mode UI needs.
//!
//! Regression (found by real-input E2E, 2026-10-26): the editor widget still reported text focus
//! in the frame where F6 / Alt+1 / Alt+3 had just requested another pane, so the editor stole
//! the focus back and the shortcuts could never leave the editor.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Explorer,
    Editor,
    Grid,
}

#[derive(Debug, Clone, Copy)]
pub struct FocusModel {
    pub current: Pane,
    requested_this_frame: bool,
}

impl FocusModel {
    pub fn new(start: Pane) -> Self {
        Self {
            current: start,
            requested_this_frame: false,
        }
    }

    /// Explicit request (shortcut, click on another pane).
    pub fn request(&mut self, pane: Pane) {
        self.current = pane;
        self.requested_this_frame = true;
    }

    /// Editor widget reports whether it still holds text focus; a click into it (no explicit
    /// request this frame) makes the editor the current pane.
    pub fn observe_editor_text_focus(&mut self, has_text_focus: bool) {
        if has_text_focus && !self.requested_this_frame {
            self.current = Pane::Editor;
        }
    }

    /// Call once at the end of every frame. Returns true when the editor must release text
    /// focus (current pane is not the editor).
    pub fn end_frame(&mut self) -> bool {
        self.requested_this_frame = false;
        self.current != Pane::Editor
    }

    /// F6 order.
    pub fn next_pane(&self) -> Pane {
        match self.current {
            Pane::Explorer => Pane::Editor,
            Pane::Editor => Pane::Grid,
            Pane::Grid => Pane::Explorer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_from_editor_is_not_stolen_back_by_text_focus() {
        let mut f = FocusModel::new(Pane::Editor);
        f.request(f.next_pane()); // F6
        f.observe_editor_text_focus(true); // editor still has text focus this frame
        assert_eq!(f.current, Pane::Grid);
        assert!(f.end_frame(), "editor must be told to release text focus");
    }

    #[test]
    fn next_frame_after_release_stays_on_grid() {
        let mut f = FocusModel::new(Pane::Editor);
        f.request(Pane::Grid);
        f.observe_editor_text_focus(true);
        f.end_frame();
        f.observe_editor_text_focus(false); // released
        assert_eq!(f.current, Pane::Grid);
        assert!(f.end_frame());
    }

    #[test]
    fn clicking_into_editor_without_request_focuses_editor() {
        let mut f = FocusModel::new(Pane::Grid);
        f.observe_editor_text_focus(true);
        assert_eq!(f.current, Pane::Editor);
        assert!(!f.end_frame());
    }

    #[test]
    fn explicit_request_for_editor_keeps_editor() {
        let mut f = FocusModel::new(Pane::Grid);
        f.request(Pane::Editor);
        f.observe_editor_text_focus(false);
        assert_eq!(f.current, Pane::Editor);
        assert!(!f.end_frame());
    }

    #[test]
    fn f6_cycles_explorer_editor_grid() {
        let mut f = FocusModel::new(Pane::Explorer);
        assert_eq!(f.next_pane(), Pane::Editor);
        f.request(Pane::Editor);
        assert_eq!(f.next_pane(), Pane::Grid);
        f.request(Pane::Grid);
        assert_eq!(f.next_pane(), Pane::Explorer);
    }

    #[test]
    fn request_flag_resets_each_frame() {
        let mut f = FocusModel::new(Pane::Editor);
        f.request(Pane::Grid);
        f.end_frame();
        // new frame: user clicks into the editor widget with no request
        f.observe_editor_text_focus(true);
        assert_eq!(f.current, Pane::Editor);
    }
}
