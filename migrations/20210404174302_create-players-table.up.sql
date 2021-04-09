-- Add up migration script here
-- Add up migration script here
CREATE TABLE players (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	stack int check (stack is null or stack >= 0),
	bet int CHECK (bet is null or bet >= 0),
	first_card_id uuid,
	second_card_id uuid,
	seat_number int check (seat_number is null or seat_number >= 0),
	status varchar(16) NOT NULL DEFAULT 'spectating',
	PRIMARY KEY (user_id, game_id),
	CONSTRAINT fk_user_id
		FOREIGN KEY (user_id)
			REFERENCES users(id),
	CONSTRAINT fk_game_id
		FOREIGN KEY (game_id)
			REFERENCES games(id),
	constraint fk_first_card_id
		foreign key (first_card_id)
			references cards(id),
	constraint fk_second_card
		foreign key (second_card_id)
			references cards(id),
	CONSTRAINT if_spectating_then_bet_and_hand_and_seat_number_are_null
		CHECK ((not status = 'spectating') or (bet is null and first_card_id  is null and second_card_id is null and stack is null and seat_number is null)),
	UNIQUE (game_id, seat_number)
);

CREATE UNIQUE INDEX players_id_seat_number_idx ON players (game_id, seat_number);
