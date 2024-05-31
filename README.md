# Rust Web Example
## Marvin Leon
### This repository contains the homework for CS410 Rust Web Development.

# Questions and Answers API

## Overview

This project is a RESTful API developed in Rust, using the Axum framework. It provides a backend for a question and answer (Q&A) application where users can interact by posting questions and attaching relevant tags. 

## Features

- **Question Struct**: Each question is represented by a struct that includes the following fields:
  - `id`: A unique identifier for the question - `string`.
  - `title`: The title of the question - `string`.
  - `content`: The detailed content of the question - `string`.
  - `tags`: Optional list of tags related to the question - (`Option<Vec<String>>`).

- **Store Struct**: This struct acts as storage for questions, using a `HashMap<String, Question>` to map question IDs (`string`) to their respective `Question` structs. It also uses a `PgPool` type connecting the API to a PostgreSQL database to allow for **persistent data storage**. All API actions interface with the PostrgeSQL database as well as the local hashmap. The local hashmap allows for faster access times and reduces the amount of times querying the database is necessary. This means only one large query is required on startup to load the hashmap.

- **PostgreSQL and SQLX**: The program now supports a persistent database using PostgreSQL. Please refer to the Installation seciton below to see how to setup your own database and send curls to add data to the DB. 

## Getting Started

### Prerequisites

- Rust
- Cargo
- PostgreSQL

## Installation

Clone the repository to your local machine:
use `cargo run` 
access the default address `127.0.0.1:3030` and be sure to use the endpointslike `127.0.0.1:3030/questions` to retrieve all questions in the PostgreSQL database.
`127.0.0.1:3030/add_question` to add a question to the PostgreSQL database.
`127.0.0.1:3000/question?start=0&end=5"` to paginate questions.


### PostgreSQL on macos
`brew install postgresql`\
`brew services start postgresql`\
`brew services stop postgresql`

### Use .env file for psql credentials
`DATABASE_URL=postgres://test_role:your_password@localhost/yourdatabase`

### Table creation
````
CREATE TABLE questions (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL, 
  tags TEXT [],
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
````
### Connect to database
`psql mydatabase`\
*Create role*\
`CREATE ROLE test_role WITH LOGIN PASSWORD 'password';`\
*Grant privileges*\
`GRANT ALL PRIVILEGES ON DATABASE mydatabase TO test_role;`\
`GRANT SELECT ON questions TO test_role;`\
`GRANT INSERT, UPDATE, DELETE ON questions TO test_role;`

### Curl to insert into the database
````
curl -X POST http://127.0.0.1:3030/add_question \
-H "Content-Type: application/json" \
-d '{"id": "1", "title": "New Question", "content": "What is Rust?", "tags": ["programming", "rust", \ "systems programming"]}' \
````
