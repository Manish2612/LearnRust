-- Your SQL goes here

CREATE TABLE users (
	id serial PRIMARY KEY,
	username VARCHAR (50) UNIQUE NOT NULL,
	password VARCHAR (50) NOT NULL,
	created_on TIMESTAMP NOT NULL
);
