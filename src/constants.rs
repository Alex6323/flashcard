pub const APP_NAME: &str = "flashcard";
pub const APP_VERSION: &str = "v0.1.0";
pub const HEADER_HEIGHT: u16 = 5;
pub const DB_NAME: &str = "progress.db";

/// Markup indicating the front side of a flashcard.
pub const MARKUP_FACE: char = '#';

/// Markup indicating that a line is meta information, and should be ignored by the
/// parser.
pub const MARKUP_META: char = '%';

/// Markup indicating a note for providing additional context.
pub const MARKUP_NOTE: char = '!';

/// Escape character in case markup and data collide.
pub const MARKUP_ESCAPE: char = '\\';

/// Markup characters
pub const MARKUP: [char; 3] = [MARKUP_FACE, MARKUP_META, MARKUP_NOTE];

/// The prompt whenever user input is required.
pub const PROMPT_INPUT: char = '>';

/// The prompt whenever additional context information is provided.
pub const PROMPT_NOTE: char = '!';

/// Number of typos allowed to successfully answer a flashcard.
pub const ALLOWED_TYPOS_PER_LINE: usize = 3;

/// Number of hints allowed to successfully answer a flashcard.
pub const ALLOWED_HINTS_PER_LINE: usize = 0;

/// Number of flashcards that can be newly added to the 1st queue.
pub const INITIAL_QUEUE_SIZE: usize = 3;

pub const STAGE1_COOLDOWN: u64 = 0;
pub const STAGE2_COOLDOWN: u64 = 120; // 2min
pub const STAGE3_COOLDOWN: u64 = 240; // 4min
pub const STAGE4_COOLDOWN: u64 = 480; // 8min
pub const STAGE5_COOLDOWN: u64 = 960; //16min
