-- Add up migration script here
CREATE TABLE IF NOT EXISTS clients (
	id uuid DEFAULT uuid_generate_v4 (),
	owner_id uuid NOT NULL,
	client_secret uuid DEFAULT uuid_generate_v4(),
	PRIMARY KEY (id),
	CONSTRAINT fk_owner_id
		FOREIGN KEY (owner_id)
			REFERENCES users(id)
);

CREATE INDEX clients_owner_id_idx ON clients (owner_id);
