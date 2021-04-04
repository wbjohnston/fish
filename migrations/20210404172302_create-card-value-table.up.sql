-- Add up migration script here
CREATE TABLE card_values (
	name char(1) PRIMARY KEY
);

INSERT INTO card_values (name) VALUES ('2'), ('3'), ('4'), ('5'), ('6'), ('7'), ('8'), ('9') ,('T'), ('J'), ('Q'), ('K'), ('A');
