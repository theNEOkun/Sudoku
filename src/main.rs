mod board;
mod app;
mod term;

use clap::Parser;
use term::Term;
use app::{App, run_app};
use board::difficulties::Difficulties;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = 1)]
    difficulty: u8,
}

#[deny(clippy::pedantic)]
fn main() {
    let args = Args::parse();
    let diff = Difficulties::from_num(args.difficulty);
    let app = App::new(diff);
    let mut terminal = Term::new();

    // create app and run it
    let res = run_app(&mut terminal, app);

    if let Err(err) = res {
        println!("{:?}", err);
    }
}
