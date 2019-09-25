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

Copy over the example .env file:

```shell
cp .env.example .env
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

## Running Tests

To run all of the tests:

```shell
cargo test
```

## Test Covearage

I created a repo on DockerHub that I'll update with each Rust version
(starting at 1.37), whose tags will match the Rust version.

In the root of the project:

```shell
docker run -it --rm --security-opt seccomp=unconfined --volume "${PWD}":/volume --workdir /volume ddimaria/rust-kcov:1.37
```

_note: converage takes a long time to run (about 30 mins)._

You can view the HTML output of the report at `target/cov/index.html`

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
