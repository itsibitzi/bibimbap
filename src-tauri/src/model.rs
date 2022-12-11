pub type DeckId = i64;

pub struct Deck {
    pub id: DeckId,
    pub name: String,
    pub card_count: i32,
}

pub struct Card {
    pub english: String,
    pub korean: String,
}
