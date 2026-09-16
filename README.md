# Rust Job Scheduler

A persistent asynchronous job scheduler built in Rust.

## Features

- REST API for job creation and management
- Persistent job storage using PostgreSQL
- Asynchronous job execution
- Concurrent worker pool
- Job retry mechanism
- Graceful shutdown
- Failure recovery
- Docker support

## Architecture

[Architecture diagram]

## Tech Stack

Rust
Tokio
Axum
PostgreSQL
SQLx
Serde

## API

POST   /jobs
GET    /jobs
GET    /jobs/:id
DELETE /jobs/:id
GET    /health

## Running Locally

...

## Design Decisions

...

## Future Improvements

...
