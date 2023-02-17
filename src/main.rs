use rand::prelude::*;
use std::io;

// Card holds the card rank
struct Card {
    rank: String,
}
impl Card {
    fn new(rank: &str) -> Card {
        Card {
            rank: rank.to_string(),
        }
    }

    // matching card value to string
    fn card_value(&self) -> i32 {
        match self.rank.as_str() {
            "Ace (A)" => 11,
            "Two (2)" => 2,
            "Three (3)" => 3,
            "Four (4)" => 4,
            "Five (5)" => 5,
            "Six (6)" => 6,
            "Seven (7)" => 7,
            "Eight (8)" => 8,
            "Nine (9)" => 9,
            _ => 10,
        }
    }
}

// Deck holds the cards to be shuffled
struct Deck {
    cards: Vec<Card>,
}
// using Deck to shuffle and deal cards
impl Deck {
    // creating a new deck of cards
    fn new_deck() -> Deck {
        let ranks = vec![
            "Ace (A)",
            "Two (2)",
            "Three (3)",
            "Four (4)",
            "Five (5)",
            "Six (6)",
            "Seven (7)",
            "Eight (8)",
            "Nine (9)",
            "Ten (10)",
            "Jack (J)",
            "Queen (Q)",
            "King (K)",
        ];

        let mut deck = Deck { cards: vec![] };

        // loop over each rank appending to the deck
        for rank in ranks.iter() {
            let card = Card::new(rank);
            deck.cards.push(card);
        }
        deck
    }

    // shuffle the deck
    fn shuffle(&mut self) {
        // create random int
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    // deal one card by removing the top card
    fn deal_one(&mut self) -> Card {
        self.cards.remove(0)
    }

    // deals two cards to the player and computer
    fn deal_cards(&mut self) -> (Vec<Card>, Vec<Card>) {
        let player_cards = vec![self.deal_one(), self.deal_one()];
        let computer_cards = vec![self.deal_one(), self.deal_one()];

        (player_cards, computer_cards)
    }
}

// counts values of cards
fn count_cards(cards: &Vec<Card>) -> i32 {
    // use fold method to get sum of cards
    let mut count = cards.iter().fold(0, |acc, card| acc + card.card_value());
    // use filter method to count number of aces
    let mut ace_count = cards.iter().filter(|c| c.rank == "Ace (A)").count();

    while count > 21 && ace_count > 0 {
        count -= 10;
        ace_count -= 1;
    }

    count
}

fn main() {
    println!("\n♠️♦️♣️♥️ *** Let's play some rusty blackjack! *** ♠️♦️♣️♥️");
    println!("\n🎲 *** Shuffling deck and dealing cards... *** 🎲");

    // create new deck and shuffle it
    let mut deck = Deck::new_deck();
    deck.shuffle();

    // deal two cards each
    let (mut player_cards, mut computer_cards) = deck.deal_cards();

    // print cards
    println!("\n> Player:");
    for card in player_cards.iter() {
        println!("{}", card.rank);
    }
    println!("\n> Computer:");
    println!("{}", computer_cards[0].rank);
    println!("Unknown (?)");

    // deal additional cards to player until they stand or go bust
    loop {
        // ask player if they want to hit
        let mut input = String::new();
        println!("\n> Do you want to hit? (y/n)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        // if yes deal a card
        if input == "y" {
            let card = deck.cards.remove(0);
            player_cards.push(card);

            println!("\n> Player:");
            for card in player_cards.iter() {
                println!("{}", card.rank);
            }

            // check if player goes bust
            if count_cards(&player_cards) > 21 {
                println!("\n> You've gone bust! Game over. ☹️");
                break;
            }

            // check if player has blackjack
            if count_cards(&player_cards) == 21 {
                println!("\n> BLACKJACK! YOU WIN! 🎉");
                break;
            }
        } else {
            // computers turn if player stands
            println!("\n*** Computers turn ***");
            while count_cards(&computer_cards) < 17 {
                let card = deck.cards.remove(0);
                computer_cards.push(card);

                println!("\n> Computer:");
                for card in computer_cards.iter() {
                    println!("{}", card.rank);
                }

                // check if computer wins
                if count_cards(&computer_cards) == 21 {
                    println!("\n> Computer got blackjack. ☹️");
                    break;
                }
                // check if computer busts
                if count_cards(&computer_cards) > 21 {
                    println!("\n> Computers gone bust! You win! 🎉");
                    break;
                }
            }
            // determine winner if no one busts or has blackjack
            let player_count = count_cards(&player_cards);
            let computer_count = count_cards(&computer_cards);

            if player_count > computer_count {
                println!(
                    "\n> You got: {}, Computer got: {}, YOU WIN! 🎉",
                    player_count, computer_count
                );
            } else if player_count < computer_count && computer_count < 21 {
                println!(
                    "\n> You got: {}, Computer got: {}, Computer wins. ☹️",
                    player_count, computer_count
                );
            } else if player_count == computer_count {
                println!(
                    "\n> You got: {}, Computer got: {}, It's a tie! 😲",
                    player_count, computer_count
                )
            } else {
                // prompt to play again
                let mut input = String::new();
                println!("\n*** Do you want to play again? (y/n) ***");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim();

                if input == "y" {
                    main(); // start new game if yes
                } else {
                    println!("\n*** Thanks for playing! 👋 ***\n")
                }
            }
            break;
        }
    }

    // prompt to play again
    let mut input = String::new();
    println!("\n*** Do you want to play again? (y/n) ***");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    if input == "y" {
        main(); // start new game if yes
    } else {
        println!("\n*** Thanks for playing! 👋 ***")
    }
}
