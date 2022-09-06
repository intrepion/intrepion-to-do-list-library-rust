pub struct Item {
    pub is_done: bool,
    pub title: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            is_done: false,
            title: "Untitled".to_string(),
        }
    }
}
