use flash::prelude::*;

use std::io::{self, Write};

fn main() {
    let cli = Cli::new();

    let mut display = Display::new();
    display.clear();
    display.print_header();

    //let cardbox = CardBox::from_file("./sample_box.txt");
    let cardbox = CardBox::from_file(cli.filepath());

    for flashcard in cardbox {
        // Print the front side of the flash card
        display.println(format!("{}", flashcard.face));
        print!("\r> ");
        io::stdout().flush().expect("error flushing stdout");

        // Read and validate user input
        let mut validator = InputValidator::new(&flashcard.back);
        display.read_input(&mut validator);

        //if validator.threshold() > 0.9_f64 {
        // number of corrections below threshold (e.g typos) -> move up a level
        // allow to set strictness for validator
        //} else {
        // this flashcard remains on current level
        //}

        if validator.is_happy() {
            display.println("Level up");
        } else {
            // Print solution
            display.println(format!("{}", flashcard.back));
            display.println("Level down");
        }

        display.println("");
        display.println("<PRESS ENTER>");
        display.wait_for_return();
        display.clear();
    }
}
