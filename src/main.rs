mod ai;
mod block;
mod ga;
mod game;
mod play;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Normal,
    Auto,
    Learning,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        None | Some(Mode::Normal) => {
            play::normal();
        }
        Some(Mode::Auto) => {
            play::auto();
        }
        Some(Mode::Learning) => {
            ga::learning();
        }
    }
}
