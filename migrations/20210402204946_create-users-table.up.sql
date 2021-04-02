CREATE TABLE IF NOT EXISTS users
(
	id uuid DEFAULT uuid_generate_v4 (),
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL,
	PRIMARY KEY (id)
);
