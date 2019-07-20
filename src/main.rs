
use flash::CardBox;
use flash::Display;

fn main() {
    let display = Display::new();
    display.clear();

    let cardbox = CardBox::from_file("./sample_box.txt");
    println!("size = {}", cardbox.size());
}
