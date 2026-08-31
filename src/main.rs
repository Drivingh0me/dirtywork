mod cli;
pub mod error;

use error::Result;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;

    run_app()?;

    Ok(())
}

fn run_app() -> Result<()> {
    println!("Worked!");
    Ok(())
}
