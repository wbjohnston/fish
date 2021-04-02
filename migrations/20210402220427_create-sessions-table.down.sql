-- Add down migration script here
CREATE TABLE sessions (
	id uuid DEFAULT uuid_generate_v4(),
	owner_id uuid NOT NULL,
	bucket json NOT NULL,
	PRIMARY KEY (id),
	CONSTRAINT fk_onwer
		FOREIGN KEY (owner_id)
			REFERENCES users(id)
);
