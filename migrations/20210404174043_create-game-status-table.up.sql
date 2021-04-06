-- Add up migration script here
CREATE TABLE game_status (
	name varchar(16),
	PRIMARY KEY(name)
);

INSERT INTO game_status (name) values ('created'), ('paused'), ('running'), ('ended');
