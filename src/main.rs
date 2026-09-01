mod cli;
mod dw_engine;
pub mod error;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;

    run_app(args)?;

    Ok(())
}

fn run_app(args: cli::Params) -> Result<()> {
    dw_analysis(board);
    println!("Worked!");
    Ok(())
}
