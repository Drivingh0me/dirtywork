use std::path::PathBuf;
use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false,
    author = "Caleb Griffin",
    version = "0.1.1",
    about = "dirtywork is a naive chess analysis engine.",
    long_about = "dirtywork uses brute force and intuition strategies\
        to analyze positions.")]
struct Args {
    // Not sure this is useful.
    #[arg(short, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    #[arg(short, action = clap::ArgAction::SetTrue)]
    play: bool,

    #[arg(short)]
    file: Option<PathBuf>,

    // Max memory in MB the engine can use.
    #[arg(short, default_value_t = 512)]
    memory: u32,

    /// Enters debug mode
    #[arg(short, value_name = "debug mode", action = clap::ArgAction::SetTrue)]
    debug: bool,
}

// Make Resolution the struct from render.rs
pub struct Params{
    pub verbose: bool,
    pub play: bool,
    pub file: Option<PathBuf>,
    pub debug: bool,
}

pub fn get_args() -> Result<Params>
{
    let args = Args::parse();

    Ok(Params {
        verbose: args.verbose,
        play: args.play,
        file: args.file,
        memory: args.memory,
        debug: args.debug,
    })
}
