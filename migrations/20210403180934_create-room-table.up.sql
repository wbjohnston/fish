-- Add up migration script here
CREATE TABLE rooms (
	id uuid DEFAULT uuid_generate_v4(),
	name TEXT NOT NULL,
	owner_id uuid NOT NULL,
	PRIMARY KEY (id),
	CONSTRAINT fk_owner
		FOREIGN KEY (owner_id)
			REFERENCES users(id)
);

CREATE INDEX rooms_owner_id_idx ON rooms (owner_id);
