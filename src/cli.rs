use std::path::PathBuf;
use clap::Parser;

use crate::error::Result;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = false,
    author = "Caleb Griffin",
    version = "0.1.2",
    about = "dirtywork is a chess analysis engine.",
    long_about = "dirtywork uses brute force and intuition strategies\
        to analyze positions.")]
pub struct Args {
    // Not sure this is useful.
    #[arg(short,
        help = "Runs the app in verbose mode",
        action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(short,
        help = "Runs the app as an interactive chess game",
        action = clap::ArgAction::SetTrue)]
    pub play: bool,

    #[arg(short,
        help = "Runs magic number search instead of app",
        action = clap::ArgAction::SetTrue)]
    pub search: bool,

    #[arg(short, help = "Optional chess game file to open")]
    pub file: Option<PathBuf>,

    // Max memory in MB the engine can use.
    #[arg(short,
        help = "Sets the max memory the ap can use",
        default_value_t = 512)]
    pub memory: u32,

    /// Enters debug mode
    #[arg(short,
        value_name = "debug mode",
        help = "Runs the app in debug mode",
        action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}

pub fn get_args() -> Result<Args>
{
    let args = Args::parse();

    Ok(args)
}
