use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub id: String,
    revision_id: String,
    pub title: String,
    pub contents: Vec<String>,
    pub headings: Vec<String>,
}

impl Document {
    pub fn new(line: &str) -> Self {
        serde_json::from_str(line).unwrap()
    }
}