//! flash - A flashcard inspired learning application for the terminal using IOTA
//! microtransactions for gamification.

#![deny(missing_docs, bad_style, unsafe_code)]

mod constants;
mod parser;
mod validator;
mod cli;

pub mod cardbox;
pub mod display;

/// Re-export of commonly used types.
pub mod prelude {
    use super::*;

    pub use cardbox::CardBox;
    pub use display::Display;
    pub use validator::InputValidator;
    pub use cli::Cli;
}
