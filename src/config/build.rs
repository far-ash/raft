use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    sources: Vec<PathBuf>,
    include: Vec<PathBuf>,
}

impl Build {
    pub fn new() -> Self {
        Self {
            files: Files {
                sources: vec![PathBuf::from("src")],
                include: vec![PathBuf::from("inc")],
            },
        }
    }
}
