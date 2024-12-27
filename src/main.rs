//! # Connect Four CLI Game
//!
//! ## Introduction
//!
//! Connect Four is a classic two-player connection game. Players take turns selecting a column to drop their colored discs into a vertically suspended grid with seven columns and six rows. The discs fall straight down, occupying the lowest available space in the chosen column.
//!
//! The objective of the game is to be the first to form a line of four discs horizontally, vertically, or diagonally.
//!
//! This implementation provides a command-line interface (CLI) version of the game. It supports up to 26 players and allows for customizable board dimensions. The default board size is 6x7. Each player is assigned a unique token (character), starting with `a` for the first player, followed by `b`, `c`, and so on for subsequent players.
//!
//! ## Usage
//!
//! Run the game with the following command:
//!
//! ```bash
//! connect_four [OPTIONS]
//! ```
//!
//! ### Options:
//!
//! - `-p`, `--players <PLAYERS> <PLAYERS>...`
//!   Specify the players participating in the game.
//!
//! - `-r`, `--rows <ROWS>`
//!   Set the number of rows on the board. [default: 6]
//!
//! - `-c`, `--cols <COLS>`
//!   Set the number of columns on the board. [default: 7]
//!
//! - `-t`, `--tokens-to-win <TOKENS_TO_WIN>`
//!   Specify the number of connected tokens required to win. [default: 4]
//!
//! - `-h`, `--help`
//!   Display usage information.
//!
//! ## Roadmap
//!
//! Planned features for future development include:
//!
//! - [ ] WebSocket support for online multiplayer functionality.
//! - [ ] An AI opponent to play against.
//!
//! ## Examples
//!
//! ```
//! let players = vec![
//!    Player::new("Alice"),
//!    Player::new("Bob"),
//! ]
//!
//! let rows = 6;
//! let cols = 7;
//! let tokens_to_win = 4;
//!
//! let mut game = Game::new(rows, cols, tokens_to_win, players);
//! game.start();
//! ```

use clap::Parser;

pub mod game;
use game::{Game, Player};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, value_parser, num_args = 2..=26, value_delimiter = ' ')]
    pub players: Vec<String>,

    #[arg(short, long, default_value = "6")]
    pub rows: usize,

    #[arg(short, long, default_value = "7")]
    pub cols: usize,

    #[arg(short, long, default_value = "4")]
    pub tokens_to_win: usize,
}

/// This is the main entry point for the Connect Four CLI game.
fn main() {
    let args = Args::parse();

    let players = args
        .players
        .iter()
        .map(|name| Player::new(name))
        .collect::<Vec<Player>>();

    Game::new(args.rows, args.cols, args.tokens_to_win, players).start();
}
