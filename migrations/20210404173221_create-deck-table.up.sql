-- Add up migration script here
CREATE TABLE decks (
	id uuid DEFAULT uuid_generate_v4(),
	PRIMARY KEY (id)
);
