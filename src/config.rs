use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, vec};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub build: Build,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    version: String,
    edition: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    sources: Vec<PathBuf>,
    include: Vec<PathBuf>,
}

#[allow(dead_code)]
impl Config {
    pub fn init(name: &str) -> String {
        let config = Config {
            project: Project {
                name: name.to_owned(),
                version: "0.1.0".to_owned(),
                edition: "2023".to_owned(),
                authors: None,
                description: None,
            },
            build: Build {
                files: Files {
                    sources: vec![PathBuf::from("src")],
                    include: vec![PathBuf::from("inc")],
                },
            },
        };
        toml::to_string_pretty(&config).unwrap()
    }

    pub fn new(path: PathBuf) -> Config {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
