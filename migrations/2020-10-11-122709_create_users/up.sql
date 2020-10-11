-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR(255) NOT NULL UNIQUE,
	password VARCHAR(255) NOT NULL,
	display_name VARCHAR(500),
	profile_picture VARCHAR(500),
	bio TEXT
);
