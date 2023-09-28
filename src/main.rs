mod command;
mod config;
mod run;

use clap::Parser;
use command::*;
use run::Run;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Command::Init(init) => init.run(),
        Command::Build(build) => println!("{}", toml::to_string_pretty(&build.build()).unwrap()),
    }
}
