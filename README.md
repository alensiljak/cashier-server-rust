# Cashier Server in Rust

This is a Cashier Server, written in Rust.

## Purpose

This is a server component that runs on a device and provides Cashier PWA with the data from the (Beancount) ledger.

Normally it is used only for a Synchronization process, after which the Cashier continues to operate offline.

With the introduction of Rust Ledger project, some of the functionality may get deprecated as Beancount is now available in WASM and therefore can run in Cashier itself.

### History

Originally, this was a Ledger-cli REST server for [Cashier](https://github.com/alensiljak/cashier) PWA.

## Related projects

Existing projects include:

- Cashier Server, Python, [repo](https://github.com/alensiljak/cashier-server-python)
- Cashier Sync, Go, [repo](https://gitlab.com/alensiljak/cashiersync-go) in Go
- Cashier Sync, Python, Github [repo](https://github.com/alensiljak/cashiersync-python) or Gitlab [repo](https://gitlab.com/alensiljak/cashiersync)
- Cashier Server, Node, [repo](https://github.com/alensiljak/cashier-server-node)
- Cashier Ledger Server, Go, [repo](https://github.com/alensiljak/cashier-ledger-server-go)

The latest update includes porting the current functionality from the Python version in 2026-03.
The current server is always paired with the actual version of Cashier:

- Cashier, Sveltekit, [repo](https://github.com/alensiljak/cashier-sveltekit)
- Cashier, Blazor, [repo](https://github.com/alensiljak/cashier-blazor)
- Cashier, Vue/2/3, [repo](https://github.com/alensiljak/cashier)

The underlying engine has included:

- Rust Ledger (.beancount-compatible)
- Beancount
- Ledger-cli

## Installation

`cargo install cashier-server`

## Run

Make sure that Rust Ledger is in the Path and is configured with the default ledger file.
Then run:

```sh
cashier-server
```

## Development

VSCode + Rust extension(s).
`cargo run` in the project directory.

## Publishing

The binary is published as a Cargo [crate](https://crates.io/crates/cashier-server).

```sh
cargo publish
```
