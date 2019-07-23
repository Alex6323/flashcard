pub const APP_NAME: &str = "FLASHCARD";
pub const APP_VERSION: &str = "v0.1.0";
pub const HEADER_HEIGHT: u16 = 5;

/// Markup indicating the front side of a flashcard.
pub const MARKUP_FACE: char = '#';

/// Markup indicating that a line is meta information, and should be ignored by the parser.
pub const MARKUP_META: char = '%';

/// Markup indicating a note for providing additional context.
pub const MARKUP_NOTE: char = '!';


/// The prompt whenever user input is required.
pub const PROMPT_INPUT: char = '>';

/// The prompt whenever additional context information is provided.
pub const PROMPT_NOTE: char = '!';

/// Escape character in case markup and data collide.
pub const ESCAPE_CHAR: char = '\\';

/// Number of typos allowed to successfully answer a flashcard.
pub const ALLOWED_TYPOS_PER_LINE: usize = 3;

/// Number of hints allowed to successfully answer a flashcard.
pub const ALLOWED_HINTS_PER_LINE: usize = 0;
