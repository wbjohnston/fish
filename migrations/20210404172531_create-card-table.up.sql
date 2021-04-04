-- Add up migration script here
CREATE TABLE cards (
	value char(1) NOT NULL,
	suit varchar(16) NOT NULL,
	PRIMARY KEY (value, suit),
	CONSTRAINT fk_value
		FOREIGN KEY (value)
			REFERENCES card_values(name),
	CONSTRAINT fk_suit
		FOREIGN KEY (suit)
			REFERENCES card_suits(name)
)
