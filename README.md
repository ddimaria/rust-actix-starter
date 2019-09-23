# Rust/Actix Starter Kit

A production-quality starter app using Actix 1.x. Includes tests.

## Packages

- `actix-web`: Actix Web Server
- `derive_more`: Error Formatting
- `dotenv`: Configuration Loader (.env)
- `envy`: Deserializes Environment Variables into a Config Struct
- `listenfd`: Listens for Filesystem Changes
- `validator`: Validates incoming Json

## Installation

Clone the repo and cd into the repo:

```shell
git clone https://github.com/ddimaria/rust-actix-starter.git
cd rust-actix-starter
```

## Running Tests

To run all of the tests:

```shell
cargo test
```

## Running the Server

To startup the server:

```shell
cargo run
```

## Autoreloading

To startup the server and autoreload on code changes:

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
