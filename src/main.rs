use flash::CardBox;
use flash::Display;

use std::io::{self, Write};

fn main() {
    let mut display = Display::new();
    display.clear();
    display.print_header();

    let cardbox = CardBox::from_file("./sample_box.txt");

    let mut input = String::new();

    for flashcard in cardbox {
        // 1. Print the front side of the flash card
        display.println(format!("TASK: {}", flashcard.face));
        print!("\r> ");
        io::stdout().flush().expect("error flushing stdout");

        // 2. Wait for user input
        input = display.read_input();
        //io::stdin().read_line(&mut input).expect("error reading input");

        // 3. Print solution
        display.println(format!("SOLUTION: {}", flashcard.back));
        display.println("<PRESS ENTER>");
        //io::stdout().flush().expect("error flushing stdout");

        display.wait_for_return();
        display.clear();
    }
}
