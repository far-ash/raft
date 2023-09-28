pub mod build;
pub mod project;

use build::Build;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub build: Build,
}

#[allow(dead_code)]
impl Config {
    pub fn init(name: &str) -> String {
        let config = Config {
            project: Project::new(name),
            build: Build::new(),
        };
        toml::to_string_pretty(&config).unwrap()
    }

    pub fn new(path: PathBuf) -> Config {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
