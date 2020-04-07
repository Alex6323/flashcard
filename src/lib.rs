//! flash - A flashcard inspired learning application for the terminal using IOTA
//! microtransactions for gamification.

#![deny(missing_docs, bad_style, unsafe_code)]

mod cardbox_parser;
mod cli;
mod common;
mod constants;
mod db;

pub mod cardbox;
pub mod display;
pub mod flashcards;
pub mod validator;

/// Re-export of commonly used types.
pub mod prelude
{
    pub use super::cardbox::{Cardbox, Envelope};
    pub use super::cli::Cli;
    pub use super::display::Display;
    pub use super::flashcards::Flashcard;
    pub use super::validator::{FlashcardValidator, InputValidator};
    pub use crossterm::Color;
}
