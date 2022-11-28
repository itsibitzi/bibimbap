pub type DeckId = i32;
pub struct Deck {
    id: DeckId,
    name: String,
    card_count: u32,
}

pub struct Card {
    english: String,
    korean: String,
}
