-- Add up migration script here
CREATE TABLE card_suits (
	name varchar(16) PRIMARY KEY
);


INSERT INTO card_suits (name) VALUES ('diamonds'), ('hearts'), ('clubs'), ('spades');
