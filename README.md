# Rust Web Example
## Marvin Leon
### This repository contains the homework for CS410 Rust Web Development.

# Questions and Answers API

## Overview

This project is a RESTful API developed in Rust, using the Axum framework. It provides a backend for a question and answer (Q&A) application where users can interact by posting questions and attaching relevant tags. 

## Features

- **Question Struct**: Each question is represented by a struct that includes the following fields:
  - `id`: A unique identifier for the question.
  - `title`: The title of the question.
  - `content`: The detailed content of the question.
  - `tags`: Optional list of tags related to the question (`Option<Vec<String>>`).

- **QuestionBase Struct**: This struct acts as a storage for questions, using a `HashMap<String, Question>` to map question IDs to their respective `Question` structs. It includes methods for initializing the question base from a JSON file, allowing persistent storage of questions.

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

Clone the repository to your local machine:
use `cargo run` 
access the default address `127.0.0.1:3030` and be sure to use the endpointslike `127.0.0.1:3030/questions`
