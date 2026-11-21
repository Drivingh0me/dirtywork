use chrono::Local;

mod cli;
mod engine;
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
    let mut game = engine::GameState::default();
    let magic = engine::initialize_magic();

    // This should be changable by the UCI.
    let depth = args.int;

    if args.search {
        println!("start time: {}", Local::now());
        engine::search_magic(magic);
        println!("end time: {}", Local::now());
    } else {
        let (mut best_move, mut eval) = engine
            ::dw_analysis(&game, depth, &magic)?;
        println!("eval: {}", eval);
        ui::print_board(best_move)?;

        // Debug code. -----
        for i in 0..40 {
            game.push(best_move);
            (best_move, eval) = engine::dw_analysis(&game, depth, &magic)?;
            println!("eval: {}", eval);
            ui::print_board(best_move)?;
        }
        //------------------
    }

    Ok(())
}
