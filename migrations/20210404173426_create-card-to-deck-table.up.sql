-- Add up migration script here
CREATE TABLE card_to_deck (
	id uuid DEFAULT uuid_generate_v4(),
	deck_id uuid NOT NULL,
	position int NOT NULL,
	value varchar(16) NOT NULL,
	suit varchar(16) NOT NULL,
	PRIMARY KEY(id)
);
