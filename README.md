# rust web practice

## Marvin Leon

# _Questions and Answers API_

## Overview

This project is a RESTful API developed in Rust, using the Axum framework. It provides a backend for a question and answer (Q&A) application where users can interact by posting questions and attaching relevant tags.

## Features

- **Question Struct**: Each question is represented by a struct that includes the following fields:

  - `id`: A unique identifier for the question - `string`.
  - `title`: The title of the question - `string`.
  - `content`: The detailed content of the question - `string`.
  - `tags`: Optional list of tags related to the question - (`Option<Vec<String>>`).

- **Store Struct**: This struct acts as storage for questions, using a `HashMap<String, Question>` to map question IDs (`string`) to their respective `Question` structs. It also uses a `PgPool` type connecting the API to a PostgreSQL database to allow for **persistent data storage**. All API actions interface with the PostgreSQL database as well as the local hashmap. The local hashmap allows for faster access times and reduces the amount of times querying the database. The PostgreSQL database ensures all data from the app is stored, even when it goes offline.

- **PostgreSQL and SQLX**: The program supports a persistent database using PostgreSQL. Please refer to the Installation section below to see how to setup your own database and send curls to add data to the DB.

- **Yew Frontend**: The program now has a frontend using the Yew framework. The frontend located in the `/rust-yew` directory uses reqwasm to utilize the backend endpoints and Yew manages and builds the necessary frontend components.

## Screenshots

_example of how to add a question to the app._
![Add question](./screenshots/add.png)

![Update question](./screenshots/update.png)
_example of how to update an existing question._


## Getting Started

### Prerequisites

- Rust
- Cargo
- PostgreSQL
- Trunk
- Wasm-Bindgen-cli
- rustup target add wasm32-unknown-unknown

## Installation

- Clone the repository to your local machine
- Install the following dependencies

```
cargo install trunk
cargo install wasm-pack
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
brew install postgresql
```

- _postgresql assumes brew has already been installed on macos system._
- _see below how to configure postgresql to setup the database server_

## Running Backend

```
cargo build
cargo run
```

- access the default address `127.0.0.1:3030` and be sure to use the endpoints like `127.0.0.1:3030/questions` to retrieve all questions in the PostgreSQL database.\
  `127.0.0.1:3030/add_question` to add a question to the PostgreSQL database.\
  `127.0.0.1:3030/question?start=0&end=1` to paginate questions.
  `127.0.0.1:3030/delete_questions/to%20be%20deleted` to delete a question (if there are spaces in the ID use % as shown).

### Curl to insert into the database

```
curl -X POST http://127.0.0.1:3030/add_question \
-H "Content-Type: application/json" \
-d '{"id": "1", "title": "New Question", "content": "What is Rust?", "tags": ["programming", "rust", \ "systems programming"]}'
```

## Running Frontend in Development

```
cargo build
trunk serve
```

## PostgreSQL on macos

`brew install postgresql`\
`brew services start postgresql`\
`brew services stop postgresql`

### Use .env file for psql credentials

`DATABASE_URL=postgres://test_role:password@localhost/mydatabase`

### Table creation

```
CREATE TABLE questions (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  tags TEXT [],
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Connect to database

`psql mydatabase`\
_Create role_\
`CREATE ROLE test_role WITH LOGIN PASSWORD 'password';`\
_Grant privileges_\
`GRANT ALL PRIVILEGES ON DATABASE mydatabase TO test_role;`\
`GRANT SELECT ON questions TO test_role;`\
`GRANT INSERT, UPDATE, DELETE ON questions TO test_role;`

# More info

Trunk is a Rust WebAssembly app bundler and build tool that handles compiling WASM and linking other resources. It automates things like setting up the correct cratetype, packaging, and serving the project. When trunk serve is executed, it performs the build process and then serves the application locally. The output in `dist` directory includes the compiled .wasm file (the application compiled into WebAssembly), the javascript glue code (responsible for loading the .wasm file), and index.html file which includes the necessary references to the javascript glue code and any other assets. For production, `trunk build` will generate the previously mentioned production ready files in the `dist` directory. In theory, we can deploy these files in the production server.
