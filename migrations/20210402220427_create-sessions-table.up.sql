CREATE TABLE sessions (
	id uuid DEFAULT uuid_generate_v4(),
	owner_id uuid NOT NULL,
	PRIMARY KEY (id),
	CONSTRAINT fk_onwer
		FOREIGN KEY (owner_id)
			REFERENCES users(id)
);

CREATE INDEX session_owner_id_idx ON sessions (owner_id);
