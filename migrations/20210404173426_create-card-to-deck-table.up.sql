-- Add up migration script here
CREATE TABLE card_to_deck (
	id uuid DEFAULT uuid_generate_v4(),
	deck_id uuid NOT NULL,
	position int NOT NULL,
	card_value char(1) NOT NULL,
	card_suit varchar(16) NOT NULL,
	CONSTRAINT fk_deck_id
		FOREIGN KEY (deck_id)
			REFERENCES decks(id),
	CONSTRAINT fk_card_value_card_suit
		FOREIGN KEY (card_value, card_suit)
			REFERENCES cards(value, suit),
	PRIMARY KEY(id)
);
