use flashcard::prelude::*;

fn main() {
    let cli = Cli::new();

    let mut display = Display::new();
    display.init();

    let mut automat = Automat::new();
    automat.init(cli.filepath());

    // Process flashcards until they all reached final stage, or their interval isn't up
    // yet
    'outer: while let Some((flashcard, current_stage)) = automat.next() {
        display.print_progress(automat.progress());

        // Print the front side of the flash card which usually describes the task
        display.println_cr(format!("{}", &flashcard.face));

        let mut list_v = ListValidator::new(&flashcard.back);
        for mut line_v in &mut list_v.validators {
            display.print_cr(format!("{} ", PROMPT_INPUT));

            // Read and validate user input
            if !display.read_input(&mut line_v) {
                break 'outer;
            }
        }

        display.println_cr("");

        // Optionally print additional notes
        if let Some(note) = &flashcard.note {
            display.cprintln_cr(format!("({})\n", note), Color::Yellow);
        }

        // If the back of the flashcard was entered correctly, increase its stage,
        // otherwise reset its stage
        if list_v.has_passed() {
            automat.increase_stage(current_stage);
            display.print_passed();
        } else {
            automat.reset_stage(current_stage);
            display.print_failed()
        }

        display.println_cr("");
        //display.println_cr("<PRESS ENTER>");
        let exit = display.wait_for_return();
        if exit {
            break 'outer;
        }

        display.redraw();
    }

    automat.save();
}
