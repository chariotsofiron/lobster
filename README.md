# Orderbook

A simple in-memory, price-time priority limit orderbook in Rust. Appropriate for exchanges with basic matching mechanics and order types.

## Example

```rust
let mut book = VecBook<SimpleOrder>::default();

book.add(SimpleOrder::sell(0, 2, 5)).for_each(drop);
book.add(SimpleOrder::sell(1, 3, 6)).for_each(drop);
book.add(SimpleOrder::sell(2, 4, 7)).for_each(drop);

book.remove(0);

let mut fills = book.add(SimpleOrder::buy(3, 6, 6));

assert_eq!(fills.next(), Some(Fill::full(1, 3, 6)));
assert_eq!(fills.next(), None);
```

## Features

- allocation free during matching
- matching returns iterator of fills
- only supports limit orders. Market, IOC, ALO, etc. can be emulated on top.
