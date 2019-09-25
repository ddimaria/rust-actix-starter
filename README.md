# Rust/Actix Starter Kit

A production-quality starter app using Actix 1.x. Includes tests and coverage.

## Features

- Actix 1.x HTTP Server
- Filesystem organized for scale
- .env for local development
- Lazy Static Config struct
- Built-in Healthcheck (includes cargo version info)
- Listers configured for TDD
- Custom Errors and HTTP Payload/Json Validation
- Working Tests throughout
- Test Coverage Reports

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
docker run -it --rm --security-opt seccomp=unconfined --volume "${PWD}":/volume --workdir /volume ddimaria/rust-kcov:1.37 --exclude-pattern=/.cargo,/usr/lib,/src/main.rs,src/server.rs
```

_note: converage takes a long time to run (about 30 mins)._

You can view the HTML output of the report at `target/cov/index.html`

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

## Endpoints

### Healthcheck

Determine if the system is healthy.

`/health`

#### Response

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/health
```

### Users

Get all users.

`GET /api/v1/user`

#### Response

```json
[
  {
    "id": "a421a56e-8652-4da6-90ee-59dfebb9d1b4",
    "first_name": "Satoshi",
    "last_name": "Nakamoto",
    "email": "satoshi@nakamotoinstitute.org"
  },
  {
    "id": "c63d285b-7794-4419-bfb7-86d7bb3ff17d",
    "first_name": "Barbara",
    "last_name": "Liskov",
    "email": "bliskov@substitution.org"
  }
]
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/api/v1/user
```
