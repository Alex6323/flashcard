//! flash - A flashcard inspired learning application for the terminal using IOTA
//! microtransactions for gamification.

#![deny(missing_docs, bad_style, unsafe_code)]

mod cli;
mod common;
mod constants;
mod db;
mod parser;
mod validator;

pub mod automat;
pub mod display;
pub mod flashcard;

/// Re-export of commonly used types.
pub mod prelude {
    pub use super::automat::{Automat, Envelope};
    pub use super::cli::Cli;
    pub use super::constants::PROMPT_INPUT;
    pub use super::display::Display;
    pub use super::flashcard::FlashCard;
    pub use super::validator::{LineValidator, ListValidator};
    pub use crossterm::Color;
}
