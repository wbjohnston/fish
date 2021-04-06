-- Add up migration script here
CREATE TABLE hands (
	id uuid DEFAULT uuid_generate_v4(),
	first_card_id uuid NOT NULL,
	second_card_id uuid NOT NULL,
	CONSTRAINT fk_first_card_id
		FOREIGN KEY (first_card_id)
			REFERENCES card_to_deck(id),
	CONSTRAINT fk_second_card_id
		FOREIGN KEY (second_card_id)
			REFERENCES card_to_deck(id),
	UNIQUE (first_card_id, second_card_id),
	PRIMARY KEY (id)
);
