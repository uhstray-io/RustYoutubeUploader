// This should use a builder pattern
pub struct Video {
    pub title: String,
    pub description: String,
    pub path: String,
    pub madeforkids: String,
    pub tags: Vec<String>,
}

impl Video {
    pub fn get_tags_for_text_input(&self) -> String {
        self.tags.join(",") + "\n"
    }
}
