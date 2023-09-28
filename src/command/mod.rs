pub mod build;
pub mod init;

use clap::{Parser, Subcommand};
use {build::BuildArgs, init::InitArgs};

#[derive(Debug, Parser)]
#[command(name = "Raft", author, version, about, long_about = "he")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initializes a Crafty project in the current directory or the specified path.
    ///
    /// The `crafty init` command is used to set up a new Crafty project. If no path is provided,
    /// it initializes the project in the current working directory. Alternatively, you can specify
    /// a custom path to create the project in a different directory.
    ///
    /// Examples:
    ///
    ///     Initialize a Crafty project in the current directory:
    ///     $ crafty init
    ///
    ///     Initialize a Crafty project in a custom directory:
    ///     $ crafty init /path/to/my_project
    ///
    /// This command will create the necessary project structure and configuration files to get
    /// started with your C/C++ project using Crafty.
    Init(InitArgs),

    Build(BuildArgs),
}
