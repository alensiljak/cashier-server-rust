# API Update

The purpose of this project is to port the API from `cashier-server-python` to Rust.

## Steps

- [x] identify the missing API endpoints between `cashier-server-python` -> `cashier-sever-rust`
- [x] add the skeleton endpoints
- [x] refactor ledger execution to use asynchronous `tokio::process`
- [x] implement environment variable support (`dotenvy`)
- [ ] rewrite each endpoint to match the Python implementation
- [ ] ensure CORS is applied

## API Discrepancies

Discrepancies in methods between the Rust and the Python server versions.

- [x] Missing `/reload` (Python uses this to refresh Beancount state).
- [x] Missing `/infrastructure/config`.
- [x] Missing `/infrastructure/accounts`.
- [x] Missing `/infrastructure/commodities`.

## Progress

Endpoint list with status update (todo, in-progress, ported).

| Endpoint                 | Status | Notes                         |
|:-------------------------|:-------|:------------------------------|
| `/`                      | ported | Refactored to async process   |
| `/hello`                 | ported |                               |
| `/ping`                  | ported |                               |
| `/reload`                | ported | Skeleton added                |
| `/infrastructure/config` | ported | Skeleton added; needs logic   |
| `/infrastructure/*`      | ported | Skeleton added; needs logic   |
