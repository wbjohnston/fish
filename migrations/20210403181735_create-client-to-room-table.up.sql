-- Add up migration script here
CREATE TABLE client_to_room (
	client_id uuid NOT NULL,
	room_id uuid NOT NULL,


	PRIMARY KEY (client_id, room_id),
	CONSTRAINT fk_client_id
		FOREIGN KEY (client_id)
			REFERENCES clients(id),
	CONSTRAINT fk_room_id
		FOREIGN KEY (room_id)
			REFERENCES rooms(id)
);
