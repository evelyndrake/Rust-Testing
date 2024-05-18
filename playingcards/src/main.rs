use rand::Rng; // Import the Rng trait from the rand module
use rand::seq::IteratorRandom; // Import the Iterator trait from the rand::Rng module

struct Card { // Define the Card struct
    suit: String,
    value: i32,
}

impl Card { // Implement the Card struct
    fn print_card(&self) { // Method to print the card
        let value_string: String = match self.value { // Match the value to a string
            1 => "Ace".to_string(),
            2..=10 => self.value.to_string(), // Range from 2 to 10
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => "Invalid".to_string(), // Default case
        };
        let suit_symbol: String = match self.suit.as_str() { // Match the suit to a symbol
            "Hearts" => "♥".to_string(),
            "Diamonds" => "♦".to_string(),
            "Clubs" => "♣".to_string(),
            "Spades" => "♠".to_string(),
            _ => "Invalid".to_string(),
        };
        println!("{} of {} {}", value_string, self.suit, suit_symbol); // Print the card
    }

    fn greater_than(&self, other: &Card) -> bool { // Method to compare two cards
        self.value > other.value
    }
}

fn generate_random_card() -> Card {
    let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"]; // Vector of suits
    let values = 1..=13; // Range of values
    let suit = suits.iter().choose(&mut rand::thread_rng()).unwrap(); // Choose a random suit
    // Explanation of the above line:
    // 1. `suits.iter()` creates an iterator over the elements of the `suits` vector.
    // 2. `choose(&mut rand::thread_rng())` randomly selects an element from the iterator.
    // 3. `unwrap()` returns the selected element from the `Option` type
    let value = rand::thread_rng().gen_range(values); // Generate a random value
    Card { // Return a new card
        suit: suit.to_string(),
        value,
    }
}

fn main() {
    for _ in 0..5 { // Compare 2 random cards 5 times
        println!("-------------------");
        let card1 = generate_random_card();
        let card2 = generate_random_card();
        card1.print_card();
        card2.print_card();
        if card1.greater_than(&card2) { // Compare the two cards
            println!("Card 1 is greater than card 2");
        } else {
            println!("Card 2 is greater than card 1");
        }
    }
}
