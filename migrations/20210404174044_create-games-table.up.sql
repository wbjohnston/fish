CREATE TABLE games (
	id uuid DEFAULT uuid_generate_v4(),
	name TEXT NOT NULL,
	deck_id uuid NOT NULL,
	owner_id uuid NOT NULL,
	button_seat_number int NOT NULL DEFAULT 0,
	active_seat_number int NOT NULL DEFAULT 0,
	pot int NOT NULL,
	PRIMARY KEY (id),
	CONSTRAINT fk_deck_id
		FOREIGN KEY (deck_id)
			REFERENCES decks(id),
	CONSTRAINT fk_owner
		FOREIGN KEY (owner_id)
			REFERENCES users(id)
);

CREATE INDEX games_owner_id_idx ON games (owner_id);
