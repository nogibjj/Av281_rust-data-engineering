use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}

pub fn draw_spade_cards(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Ace of Spades".to_string(),
        "2 of Spades".to_string(),
        "3 of Spades".to_string(),
        "4 of Spades".to_string(),
        "5 of Spades".to_string(),
        "6 of Spades".to_string(),
        "7 of Spades".to_string(),
        "8 of Spades".to_string(),
        "9 of Spades".to_string(),
        "10 of Spades".to_string(),
        "Jack of Spades".to_string(),
        "Queen of Spades".to_string(),
        "King of Spades".to_string(),
    ];

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}
