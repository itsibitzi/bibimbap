CREATE TABLE decks (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name_display    TEXT NOT NULL,
    -- Names can be in English or Korean so we should support both
    name_decomposed TEXT NOT NULL,
);

CREATE INDEX decks_name_search ON decks(name_decomposed);


CREATE TABLE cards (
    korean_display    TEXT NOT NULL,
    -- Decomposed korean characters for prefix searching
    korean_decomposed TEXT NOT NULL,
    english           TEXT NOT NULL,
    deck_id           INTEGER NOT NULL,
    FOREIGN KEY (deck_id) REFERENCES decks(id)
);

-- We want to support searching for either Korean or English
CREATE INDEX cards_korean_search ON cards(korean_decomposed);
CREATE INDEX cards_english_search ON cards(english);
