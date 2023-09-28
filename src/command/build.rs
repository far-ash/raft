use std::env;
use std::path::PathBuf;

use crate::config::Config;
use crate::run::Run;
use clap::Args;

#[derive(Debug, Args)]
pub struct BuildArgs {
    #[arg(default_value_t = String::from("debug"))]
    kind: String,
    #[arg(short, long)]
    path: Option<PathBuf>,
}

impl BuildArgs {
    pub fn build(&self) -> Config {
        let path = self.path.clone().map_or(
            env::current_dir().map(|p| p.join("Raft.toml")).unwrap(),
            |p| p,
        );
        let config = Config::new(path);
        config
    }
}

impl Run for BuildArgs {
    fn run(&self) {
        self.build();
    }
}
