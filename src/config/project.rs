use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    version: String,
    edition: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            edition: "2021".to_string(),
            authors: None,
            description: None,
        }
    }
}
