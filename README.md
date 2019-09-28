# Rust/Actix Starter Kit

[![Build Status](https://travis-ci.com/ddimaria/rust-actix-starter.svg?branch=master)](https://travis-ci.com/ddimaria/rust-actix-starter)

A production-quality starter app using Actix 1.x. Includes tests and coverage.

## Features

- Actix 1.x HTTP Server
- Filesystem organized for scale
- .env for local development
- Lazy Static Config struct
- Built-in Healthcheck (includes cargo version info)
- Listeners configured for TDD
- Custom Errors and HTTP Payload/Json Validation
- Working Tests throughout
- Test Coverage Reports
- Dockerfile for Running the Server in a Container

## Packages

- `actix-web`: Actix Web Server
- `derive_more`: Error Formatting
- `dotenv`: Configuration Loader (.env)
- `envy`: Deserializes Environment Variables into a Config Struct
- `listenfd`: Listens for Filesystem Changes
- `validator`: Validates incoming Json
- `kcov`: Coverage Analysis

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

_note: converage takes a long time to run (up to 30 mins)._

You can view the HTML output of the report at `target/cov/index.html`

## Docker

To build a Docker image of the application:

```shell
docker build -t rust_actix_starter .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name rust_actix_starter rust_actix_starter
```

## Endpoints

### Healthcheck

Determine if the system is healthy.

`GET /health`

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

### Get All Users

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

### Get a User

`GET /api/v1/user/{id}`

#### Request

| Param | Type | Description   |
| ----- | ---- | ------------- |
| id    | Uuid | The user's id |

#### Response

```json
{
  "id": "a421a56e-8652-4da6-90ee-59dfebb9d1b4",
  "first_name": "Satoshi",
  "last_name": "Nakamoto",
  "email": "satoshi@nakamotoinstitute.org"
}
```

Example:

```shell
curl -X GET http://127.0.0.1:3000/api/v1/user/a421a56e-8652-4da6-90ee-59dfebb9d1b4
```

#### Response - Not Found

`404 Not Found`

```json
{
  "errors": ["User c63d285b-7794-4419-bfb7-86d7bb3ff17a not found"]
}
```

### Create a User

`POST /api/v1/user`

#### Request

| Param      | Type   | Description              | Required | Validations           |
| ---------- | ------ | ------------------------ | :------: | --------------------- |
| first_name | String | The user's first name    |   yes    | at least 3 characters |
| last_name  | String | The user's last name     |   yes    | at least 3 characters |
| email      | String | The user's email address |   yes    | valid email address   |

```json
{
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

#### Response

```json
{
  "id": "0c419802-d1ef-47d6-b8fa-c886a23d61a7",
  "first_name": "Linus",
  "last_name": "Torvalds",
  "email": "torvalds@transmeta.com"
}
```

Example:

```shell
curl -X POST \
  http://127.0.0.1:3000/api/v1/user \
  -H 'Content-Type: application/json' \
  -d '{
    "first_name": "Linus",
    "last_name": "Torvalds",
    "email": "torvalds@transmeta.com"
}'
```

#### Response - Validation Errors

`422 Unprocessable Entity`

```json
{
  "errors": [
    "first_name is required and must be at least 3 characters",
    "last_name is required and must be at least 3 characters",
    "email must be a valid email"
  ]
}
```

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
