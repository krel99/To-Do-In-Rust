CREATE TABLE to_do (
   id SERIAL PRIMARY KEY,
   title VARCHAR NOT NULL,
   status VARCHAR NOT NULL,
   date timestamp NOT NULL DEFAULT NOW()
);