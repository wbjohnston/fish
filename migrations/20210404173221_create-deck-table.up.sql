-- Add up migration script here
CREATE TABLE decks (
	id uuid DEFAULT uuid_generate_v4(),
	position int NOT NULL DEFAULT 0,
	PRIMARY KEY (id)
);
