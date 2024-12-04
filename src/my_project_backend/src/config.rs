use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize)]
pub struct Config {
    pub max_tags: u8,
    pub max_content_length: u16,
    pub max_title_length: u8,
    pub tags: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_tags: 3,
            max_content_length: 2048,
            max_title_length: 255,
            tags: Vec::new()
        }
    }
}
