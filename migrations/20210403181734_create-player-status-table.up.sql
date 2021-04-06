-- Add up migration script here


CREATE TABLE player_status (
	name VARCHAR(16),
	PRIMARY KEY (name)
);

INSERT INTO player_status ("name") VALUES ('playing'), ('folded'), ('standing'), ('spectating');
