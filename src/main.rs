fn main() {
    use card_games::deck::Deck;

    let mut deck = Deck::new_standard_french_deck();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}