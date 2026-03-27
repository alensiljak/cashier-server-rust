# API Update

The purpose of this project is to port the API from `cashier-server-python` (Python) to `cashier-sever-rust` (Rust).

## Steps

- [x] identify the missing API endpoints between `cashier-server-python` -> `cashier-sever-rust`
- [x] add the skeleton endpoints
- [x] refactor ledger execution to use asynchronous `tokio::process`
- [x] implement environment variable support (`dotenvy`)
- [x] rewrite each endpoint to match the Python implementation
- [x] ensure CORS is applied

## API Discrepancies

Discrepancies in methods between the Rust and the Python server versions.

- [x] Missing `/reload` (Python uses this to refresh Beancount state).
- [x] Missing `/infrastructure/config`.
- [x] Missing `/infrastructure/accounts`.
- [x] Missing `/infrastructure/commodities`.

## Progress

Endpoint list with status update (todo, in-progress, ported).

| Endpoint                 | Status  | Notes                        |
|:-------------------------|:------- |:---------------------------- |
| `/`                      | ✅      |                              |
| `/hello`                 | ✅      |                              |
| `/ping`                  | ✅      |                              |
| `/reload`                | ✅      | Not needed with a CLI engine |
| `/infrastructure/config` | ✅      | Ported from Python           |
| `/infrastructure/*`      | ✅      | Ported from Python           |
