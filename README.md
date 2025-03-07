# cov

A code coverage visualization tool.

## Tenets

The following tenets guide the development of `cov` and its features:

1. **Backwards compatibility**: `cov` should be able to consume its own data from previous versions.
   We do not commit to supporting downgrading the tool, but we do commit to supporting the data format.
2. **Opinionated**: `cov` will be useful in the exact ways we see it to be useful.
3. **Open source**: `cov` will be open source and free to use. Now and forever. Anyone can contribute, fork, or modify `cov` however they see fit, even if that means creating a proprietary version.
4. **Simple**: `cov` will be simple to use and understand. Its UI, CLI, API, and deployment will all be simple and straight-forward.

## Architecture

`cov` is divided into two applications: the `cov` binary (for the server and CLI client) and the `cov-web` React application (for the web UI).
The web UI is a single-page application that communicates with the server via a REST API.

The server is responsible for:

- Parsing coverage data from various sources (e.g. JUnit XML, Go's built-in coverage tool, etc.).
- Storing coverage data in a database. The database interactions are behind an actor-model, such that the database can be swapped out for another implementation at any point.
- Serving coverage data to the web UI.
- Authenticating users and limiting access to repositories they have access to.
- Collecting and serving metrics to observe the health of the system.

A report is a single set of coverage results from a test suite. It is tagged by a repository, commit, unique ID, and tags (e.g. `integration`, `e2e`, or `federal` to differentiate how it was tested).
This is all stored in the database. Reports that reach 30 days of age are automatically depopulated, i.e. removing line-level data and removing reports from branches that are not primary branches.

The server is intended to be horizontally scalable, and to be a long-running process that can be deployed in a Docker or Kubernetes cluster.

The entire server is designed with the actor-model in mind. This means that the server is a collection of actors that communicate with each other via message passing.
Some messages may be stored in the database, such that each server can claim it by locking a row. This is to ensure that the server can be horizontally scaled.
The reason for this is to ensure the CI/CD flow is not blocked by the server being slow, while also proactively processing data before a human needs it, or some bot requests it.

## Intended deployment

This is intended to be deployed in a Docker or Kubernetes container. The server is stateless, connecting to a stateful database.
The PostgreSQL database can be split into a write and read replica; multiple read replicas are supported via PgBouncer or PgCat.

The server and frontend are both intended to be behind a reverse proxy, such as Nginx or Caddy. As such, no HTTPS support is provided.
If you want to be safer with your data between the load-balancer and the individual servers, you can deploy pods with a sidecar that handles the TLS termination.
The server does not support Unix sockets, only HTTP over TCP/IP.

## Development

The backend is written in Rust, and the frontend is written in TypeScript.

To build the platform, use Just: `just build`. This will build the backend and frontend.
There are more recipes in the justfile: run `just -l` to see them and their assumed dependencies.

## Licence

`cov` is licensed under the MIT licence. See the [LICENCE](LICENCE) file for more information.
