# Fuel Map

Backend for project fuel-map.

## Requirements

* `docker` with `compose`.
* `Rust >= 1.58`.
* `cargo`.
* `sqlx-cli` (Can be installed using `cargo install sqlx-cli`).

## Compilation

The project uses `sqlx` for interaction and migration of the Postgresql database, and performs
compile-time checks of queries. A migrated and running database needs to be present when
compiling the project using `cargo`.

1. Copy the `.env.example` file to `.env` and make modifications if necessary.
1. Start the Posgres database server with `docker compose up`.
1. Run migrations `sqlx migrate run`.
1. Compile with `cargo build [--release]`.

## Documentation

Endpoints are documented using an OpenAPI 3.1 schema, using Stoplight Studio for
editing and testing. The schema can be found in `docs/fuel_map.yaml`.
