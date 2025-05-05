# cov

A service to visualize and browse your code coverage.

## Building

To build the project, you need `go` and [`just`](https://just.systems/man/en/packages.html) installed. Run `just build`.
<!-- TODO: Frontend -->

You can also list all recipes with `just -l`.

When developing, you need:

* `go` (<https://go.dev/doc/install>)
* `just` (<https://just.systems/man/en/packages.html>)
* `uvx` (from `uv`: <https://docs.astral.sh/uv/>)
* `actionlint` (<https://github.com/rhysd/actionlint>)
* `goimports` (<https://pkg.go.dev/golang.org/x/tools/cmd/goimports>)
* `docker` & `docker compose` (<https://docs.docker.com/engine/install/>, must be accessible sudo-less; [Podman](https://podman.io/docs/installation) probably also works if aliased to `docker`)

You may want to run `just setup-precommit` to have some fast linters running on every commit.

## Alternatives

These tools may be great alternatives to this tool for you.
I think they're valuable parts of the code quality community, and as such want to make sure they're known.
I even go so far as to use them for this project, because they are good at what they do for what I need here.

* [CodeCov by Sentry](https://about.codecov.io/)
* [Coveralls](https://coveralls.io/)

## Licence

This project is licensed under the BSD 3-Clause Licence.
This is a very permissive licence that grants you permission to do practically anything with the software.
Refer to the terms for specifics.