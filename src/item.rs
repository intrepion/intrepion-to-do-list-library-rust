pub struct Item {
    pub title: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
        }
    }
}
