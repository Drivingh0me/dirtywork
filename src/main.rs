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
    let game = dw_engine::GameState::default();
    // let best_move = dw_engine::BoardState::default();
    // let eval: f32 = 0;
    let (best_move, eval) = dw_analysis(game, 5);
    println!("eval: {}", eval);
    Ok(())
}
