-- Your SQL goes here
CREATE TABLE tasks (
	id serial NOT NULL,
	title character varying(255) NOT NULL,
	description character varying(255) NOT NULL,
	status character varying(255) NOT NULL,
	removed boolean NOT NULL,
	CONSTRAINT task_pkey PRIMARY KEY (id)
);