# Orderbook

A simple limit orderbook in under 100 lines of Rust.

Uses an array of `VecDeque`s for O(1) level lookup and O(k) matching.


## Linting

```shell
cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
```