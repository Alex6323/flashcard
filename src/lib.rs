//! flash - A flashcard inspired learning application for the terminal using IOTA
//! microtransactions for gamification.

#![deny(missing_docs, bad_style, unsafe_code)]

mod constants;
mod parser;

pub mod cardbox;
pub mod display;

pub use cardbox::CardBox;
pub use display::Display;
