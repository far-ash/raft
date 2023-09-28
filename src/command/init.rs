use crate::config::Config;
use crate::run::Run;
use clap::Args;
use color_print::cformat;
use std::{env, fs, io::ErrorKind, path::PathBuf};

#[derive(Debug, Args)]
pub struct InitArgs {
    pub path: Option<PathBuf>,
}

impl InitArgs {
    const FILES: [&str; 4] = ["Raft.toml", "raft/build.ninja", "src/main.c", "inc/main.h"];
    const DIRECTORIES: [&str; 4] = ["", "raft", "inc", "src"];

    fn path(&self) -> PathBuf {
        match self.path.clone() {
            None => match env::current_dir() {
                Ok(p) => p,
                Err(e) => raft::raise(e),
            },
            Some(p) => p,
        }
    }

    fn dirs(&self) {
        let path = self.path();

        for dir in Self::DIRECTORIES {
            if let Err(e) = fs::create_dir(&path.join(dir)) {
                if !dir.is_empty() {
                    PathBuf::from(dir)
                } else {
                    path.clone()
                };
                match e.kind() {
                    ErrorKind::AlreadyExists => (),
                    _ => raft::raise(e),
                };
            }
        }
    }

    fn files(&self) {
        let path = self.path();

        for file in Self::FILES {
            let content = fs::read_to_string(String::from("init/") + file).unwrap();
            let result = fs::write(&path.join(file), content);

            if let Err(e) = result {
                raft::raise(e);
            }
        }
    }

    fn config(&self) {
        let config = Config::init(self.path().to_str().unwrap());
        fs::write(self.path().join("Raft.toml"), config).unwrap();
    }
}

impl Run for InitArgs {
    fn run(&self) {
        self.dirs();
        self.files();
        self.config();
        eprintln!(
            "{}",
            cformat!(
                "\t<g><s>Created</></> binary (application), <k>`<dim>{}</>`</> project",
                self.path().to_str().unwrap()
            )
        )
    }
}
