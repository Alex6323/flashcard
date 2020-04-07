use crossterm::Color;

pub const APP_NAME: &str = "flashcard";
pub const APP_VERSION: &str = "0.1.0";

pub const DB_NAME: &str = "progress.db";

pub const HEADER_HEIGHT: u16 = 2;

/// Markup indicating the front side of a flashcard.
pub const MARKUP_FACE: char = '#';

/// Markup indicating that a line is a comment, and should be ignored by the
/// parser.
pub const MARKUP_COMMENT: char = '%';

/// Markup indicating a note for providing additional context.
pub const MARKUP_NOTE: char = '!';

/// Escape character in case markup and data collide.
pub const MARKUP_ESCAPE: char = '\\';

/// Markup characters
pub const MARKUP: [char; 3] = [MARKUP_FACE, MARKUP_COMMENT, MARKUP_NOTE];

/// The prompt whenever user input is required.
pub const PROMPT_INPUT: char = '>';
pub const PROMPT_WIDTH: u16 = 1;

/// The prompt whenever additional context information is provided.
pub const PROMPT_NOTE: char = '!';

/// Number of typos allowed to successfully answer a flashcard.
pub const ALLOWED_TYPOS_PER_LINE: usize = 3;

/// Number of hints allowed to successfully answer a flashcard.
pub const ALLOWED_HINTS_PER_LINE: usize = 0;

/// Number of revealed characters when requiring a hint.
pub const NUM_REVEALED_CHARS_IN_HINT: usize = 2;

/// Number of flashcards that can be newly added to the 1st queue.
pub const INITIAL_QUEUE_SIZE: usize = 3;

/// Key to press for quitting the program.
pub const PROGRAM_QUIT_KEY: char = 'q';
pub const PROGRAM_PEEK_KEY: char = 'p';

pub const STAGE1_COOLDOWN: u64 = 0;
#[cfg(debug_assertions)]
pub const STAGE2_COOLDOWN: u64 = 120; // 2min
#[cfg(not(debug_assertions))]
pub const STAGE2_COOLDOWN: u64 = 3600; // 1h
#[cfg(debug_assertions)]
pub const STAGE3_COOLDOWN: u64 = 240; // 4min
#[cfg(not(debug_assertions))]
pub const STAGE3_COOLDOWN: u64 = 21600; // 6h
#[cfg(debug_assertions)]
pub const STAGE4_COOLDOWN: u64 = 480; // 8min
#[cfg(not(debug_assertions))]
pub const STAGE4_COOLDOWN: u64 = 86400; // 24h
#[cfg(debug_assertions)]
pub const STAGE5_COOLDOWN: u64 = 960; // 16min
#[cfg(not(debug_assertions))]
pub const STAGE5_COOLDOWN: u64 = 604800; // 1w

pub const BG_COLOR: Color = Color::Cyan;
pub const FG_COLOR: Color = Color::Black;

pub const TYPE_WRITE_THE_LINE: usize = 1;
pub const TYPE_FILL_THE_BLANK: usize = 2;

pub const BLANK_INDICATOR: char = '_';
