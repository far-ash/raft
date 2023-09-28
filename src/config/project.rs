use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
}
