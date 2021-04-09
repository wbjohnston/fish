-- Add up migration script here
CREATE TABLE cards (
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

CREATE UNIQUE INDEX cards_deck_id_position_idx ON cards (deck_id, position);
