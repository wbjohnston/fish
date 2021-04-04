-- Add up migration script here


CREATE TABLE game_status (
	name VARCHAR(16),
	PRIMARY KEY (name)
);

INSERT INTO game_status ("name") VALUES ('folded'), ('standing'), ('eliminated'), ('spectating');
