use flash::prelude::*;

fn main() {
    let cli = Cli::new();

    let mut display = Display::new();
    display.clear();
    display.print_header();

    let cardbox = CardBox::from_file(cli.filepath());

    for flashcard in cardbox {
        // Print the front side of the flash card which usually describes the task
        display.println_cr(format!("{}", flashcard.face));

        let mut list_v = ListValidator::new(flashcard.back);
        for mut line_v in &mut list_v.validators {
            display.print_cr(format!("{} ", PROMPT));

            // Read and validate user input
            display.read_input(&mut line_v);
        }

        display.println_cr("");
        if list_v.is_happy() {
            display.println_cr("Level up");
        } else {
            display.println_cr("Level down");
        }

        display.println_cr("");
        display.println_cr("<PRESS ENTER>");
        display.wait_for_return();
        display.clear_except_header();
    }
}
