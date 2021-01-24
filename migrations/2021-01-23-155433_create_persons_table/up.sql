CREATE TABLE persons (
   id SERIAL PRIMARY KEY,
   first_name VARCHAR(191) NOT NULL,
   last_name VARCHAR(191) NOT NULL,
   email VARCHAR(191) NOT NULL,
   created_at TIMESTAMP NOT NULL,
   updated_at TIMESTAMP NOT NULL
);