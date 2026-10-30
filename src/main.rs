use chrono::Local;

mod cli;
mod dw_engine;
mod ui;
pub mod error;

use error::Result;
use error::Error;

fn main() -> Result<()> {
    let args = cli::get_args()?;

    run_app(args)?;

    Ok(())
}

fn run_app(args: cli::Args) -> Result<()> {
    let game = dw_engine::GameState::default();
    let magic = dw_engine::initialize_magic();

    // This should be changable by the UCI.
    let depth = 3;

    if args.search {
        println!("start time: {}", Local::now());
        dw_engine::search_magic(magic);
        println!("end time: {}", Local::now());
    } else {
        let (best_move, eval) = dw_engine::dw_analysis(game, depth, magic)?;
        println!("eval: {}", eval);
        ui::print_board(best_move)?;
    }

    Ok(())
}
