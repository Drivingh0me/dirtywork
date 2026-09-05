mod cli;
mod dw_engine;
mod ui;
pub mod error;

use error::Result;
use error::Error;

fn main() -> Result<()> {
    let args: cli::Params = cli::get_args()?;

    run_app(args)?;

    Ok(())
}

fn run_app(args: cli::Params) -> Result<()> {
    let game = dw_engine::GameState::default();
    let (best_move, eval) = dw_engine::dw_analysis(game, 5)?;
    println!("eval: {}", eval);
    // dbg!(best_move);
    ui::print_board(best_move)?;
    Ok(())
}
