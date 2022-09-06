pub struct Item {
    pub is_done: bool,
    pub is_visible: bool,
    pub title: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            is_done: false,
            is_visible: false,
            title: "Untitled".to_string(),
        }
    }
}
