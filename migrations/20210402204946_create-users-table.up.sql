CREATE TABLE IF NOT EXISTS users
(
	id uuid DEFAULT uuid_generate_v4 (),
	role TEXT NOT NULL default 'user',
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL,
	PRIMARY KEY (id)
);
