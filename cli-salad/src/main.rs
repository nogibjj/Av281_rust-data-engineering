use clap::Parser;
use cli_salad::create_fruit_salad;
use cli_salad::draw_spade_cards;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    create_fruit_salad(num_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
    let num_cards = opts.number;

    // Create the hand of cards
    draw_spade_cards(num_cards);

    // Print the hand of cards in human readable format with a count of cards
    println!(
        "Drew a hand with {} spade cards: {:?}",
        num_cards,
        draw_spade_cards(num_cards)
    );
}
