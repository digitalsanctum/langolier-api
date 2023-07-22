# langolier-api

API for Langolier, a personal digital garden and AI assistant.

## Build

To build the project, run:

```shell
cargo build
```

## Container

To build the container, run:

```shell
docker build -t langolier-api .
```

## Run

To run the project, run:

```shell
docker compose up
```

## NATS

This project uses NATS for messaging. To run NATS locally, use the following command:

```shell
docker compose up -d nats
```

## Dependencies

To find outdated dependencies, run:

```shell
cargo outdated -R
```

## References

Here are reference docs for the crates used in this project:

- [axum docs](https://docs.rs/axum/latest/axum/) - web framework that focuses on ergonomics and modularity.
- [sqlx docs](https://docs.rs/sqlx/latest/sqlx/) - an async, pure Rust SQL crate featuring compile-time checked
  queries without a DSL. Supports PostgreSQL, MySQL, and SQLite.
- [tracing docs](https://docs.rs/tracing/latest/tracing/) - a framework for instrumenting Rust programs to collect
  structured, event-based diagnostic information.
- [serde docs](https://docs.rs/serde/latest/serde/) - framework for serializing and deserializing Rust data
  structures efficiently and generically.
