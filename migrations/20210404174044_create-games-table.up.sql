CREATE TABLE games (
	id uuid DEFAULT uuid_generate_v4(),
	name TEXT NOT NULL,
	deck_id uuid NOT NULL,
	owner_id uuid NOT NULL,
	button_seat_number int NOT NULL DEFAULT 0,
	active_seat_number int NOT NULL DEFAULT 0,
	last_to_bet_seat_number int,
	pot int NOT NULL DEFAULT 0,
	phase TEXT NOT NULL default 'preflop',
	flop_1_card_id uuid,
	flop_2_card_id uuid,
	flop_3_card_id uuid,
	turn_card_id uuid,
	river_card_id uuid,
	status varchar(16) NOT NULL DEFAULT 'created',
	constraint fk_flop_1_card_id
		foreign key (flop_1_card_id)
			references card_to_deck(id),
	constraint fk_flop_2_card_id
		foreign key (flop_2_card_id)
			references card_to_deck(id),
	constraint fk_flop_3_card_id
		foreign key (flop_3_card_id)
			references card_to_deck(id),
	constraint fk_turn_card_id
		foreign key (turn_card_id)
			references card_to_deck(id),
	constraint fk_river_card_id
		foreign key (river_card_id)
			references card_to_deck(id),
	CONSTRAINT fk_deck_id
		FOREIGN KEY (deck_id)
			REFERENCES decks(id),
	CONSTRAINT fk_owner
		FOREIGN KEY (owner_id)
			REFERENCES users(id),
	PRIMARY KEY (id)
);

CREATE INDEX games_owner_id_idx ON games (owner_id);
