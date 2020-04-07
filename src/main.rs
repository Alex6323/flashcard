use flashcard::prelude::*;

fn main()
{
    let cli = Cli::new();

    let mut display = Display::new();
    display.init();

    let mut cardbox = Cardbox::new();
    cardbox.init(cli.filepath());

    // Process flashcards until they all reached final stage, or their interval isn't up
    // yet
    'outer: while let Some((flashcard, current_stage)) = cardbox.next() {
        //display.print_progress(cardbox.progress());
        cardbox.display_progress(&mut display);

        // Print the front side of the flash card which usually describes the task
        flashcard.display_face(&mut display);

        // Iterate all lines on the back of the flashcard. Each line needs:
        // - line input validator
        // - line set-up (eg. prompt, line with blanks)
        // - line input locations (valid cursor positions)

        for (mut input_validator, )
        let mut card_validator = flashcard.get_validator();

        for (i, mut input_validator) in
            &mut card_validator.validators.iter_mut().enumerate()
        {
            flashcard.display_context(&mut display, i);

            let valid_locations = flashcard.get_valid_locations(&mut display, i);

            // Read and validate user input.
            if !display.read_input_blanks(&mut input_validator, valid_locations) {
                break 'outer;
            }
        }

        /*
        for (input_validator, line_setup) in &flashcard {
            // Print line-setup
            self.print_cr(format!("{} {}", PROMPT_INPUT, context));
        }
        */

        display.println_cr("");

        // Optionally print additional notes
        flashcard.display_note(&mut display);

        // If the back of the flashcard was entered correctly, increase its stage,
        // otherwise reset its stage
        if card_validator.passed() {
            cardbox.increase_stage(current_stage);
            display.print_passed();
        } else {
            cardbox.reset_stage(current_stage);
            display.print_failed()
        }

        //display.println_cr("");

        if display.wait_for_return() {
            break 'outer;
        }

        display.redraw();
    }

    cardbox.save();
}
