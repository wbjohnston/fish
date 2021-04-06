-- Add up migration script here
-- Add up migration script here
CREATE TABLE players (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	stack int NOT NULL,
	hand_id uuid,
	seat_number int,
	status varchar(16) NOT NULL,
	PRIMARY KEY (user_id, game_id),
	CONSTRAINT fk_user_id
		FOREIGN KEY (user_id)
			REFERENCES users(id),
	CONSTRAINT fk_game_id
		FOREIGN KEY (game_id)
			REFERENCES games(id),
	CONSTRAINT fk_hand_id
		FOREIGN KEY (hand_id)
			REFERENCES hands(id),
	CONSTRAINT fk_player_status
		FOREIGN KEY (status)
			REFERENCES player_status(name)
);

CREATE INDEX players_id_seat_number_idx ON players (game_id, seat_number);
