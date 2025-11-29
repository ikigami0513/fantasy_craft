#[derive(Debug, Clone, Copy)]
pub struct InputFocus {
    pub is_captured_by_ui: bool
}

impl Default for InputFocus {
    fn default() -> Self {
        Self {
            is_captured_by_ui: false
        }
    }
}
