-- Add up migration script here
CREATE TABLE card_to_deck (
	id uuid DEFAULT uuid_generate_v4(),
	deck_id uuid NOT NULL,
	position int NOT NULL,
	value varchar(16) NOT NULL,
	suit varchar(16) NOT NULL,
	CONSTRAINT fk_deck_id
		FOREIGN KEY (deck_id)
			REFERENCES decks(id),
	PRIMARY KEY(id)
);

CREATE UNIQUE INDEX card_to_deck_deck_id_position_idx ON card_to_deck (deck_id, position);
