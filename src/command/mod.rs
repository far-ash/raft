pub mod build;
pub mod init;

use clap::{Parser, Subcommand};
use {build::BuildArgs, init::InitArgs};

#[derive(Debug, Parser)]
#[command(name = "Raft", author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initializes a C (Raft) project in the current directory or the specified path.
    ///
    /// The `Raft init` command is used to set up a new Raft project. If no path is provided,
    /// it initializes the project in the current working directory. Alternatively, you can specify
    /// a custom path to create the project in a different directory.
    ///
    /// Examples:
    ///
    ///     Initialize a Raft project in the current directory:
    ///     $ raft init
    ///
    ///     Initialize a Raft project in a custom directory:
    ///     $ raft init /path/to/my/project
    ///
    /// This command will create the necessary project structure and configuration files to get
    /// started with your C/C++ project using Raft.
    Init(InitArgs),

    Build(BuildArgs),
}
