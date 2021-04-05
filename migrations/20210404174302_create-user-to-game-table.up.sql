-- Add up migration script here
-- Add up migration script here
CREATE TABLE user_to_game (
	user_id uuid NOT NULL,
	game_id uuid NOT NULL,
	stack int NOT NULL,
	hand_id uuid NOT NULL,
	seat_number int NOT NULL,
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
	CONSTRAINT fk_game_status
		FOREIGN KEY (status)
			REFERENCES game_status(name)
);

CREATE INDEX user_to_game_id_seat_number_idx ON user_to_game (game_id, seat_number);
