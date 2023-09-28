use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub sources: Vec<PathBuf>,
    pub include: Vec<PathBuf>,
}
