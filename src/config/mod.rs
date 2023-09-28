pub mod build;
pub mod generate;
pub mod project;

use build::{Build, Files};
use generate::Generate;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub build: Build,
    pub generate: Generate,
}

#[allow(dead_code)]
impl Config {
    pub fn new(path: PathBuf) -> Config {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn init(name: &str) -> String {
        let config = Config {
            project: Project {
                name: name.into(),
                version: "0.1.0".into(),
                edition: "2021".into(),
                authors: None,
                description: None,
            },
            build: Build {
                files: Files {
                    sources: vec!["src".into()],
                    include: vec!["inc".into()],
                },
            },
            generate: Generate {
                target: "make".into(),
            },
        };
        toml::to_string_pretty(&config).unwrap()
    }
}
