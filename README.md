# cashier-server
Ledger-cli REST server for [Cashier](https://github.com/alensiljak/cashier) PWA.

Cashier Server acts as a mediator between Cashier PWA and Ledger CLI, forwarding queries to Ledger and the results to Cashier. Used for synchronizing the ledger data in Cashier.

This version of the Cashier Server is implemented in Rust.

Previous implementations:

- [Cashier Ledger Server](https://github.com/alensiljak/cashier-ledger-server-go) in Go
- [CashierSync](https://gitlab.com/alensiljak/cashiersync) in Python
- [CashierSync Go](https://gitlab.com/alensiljak/cashiersync-go) in Go

# Development

VSCode + Rust extension(s).
`cargo run` in project directory.

# Publishing

`cargo publish`
