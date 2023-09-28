use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub files: Files,
    pub directories: Directories,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directories {
    pub sources: Vec<PathBuf>,
    pub include: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub sources: Option<Vec<PathBuf>>,
    pub include: Option<Vec<PathBuf>>,
}
