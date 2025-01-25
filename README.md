# Orderbook

A simple in-memory, price-time priority limit orderbook in Rust. Appropriate for exchanges with simple matching mechanics and order types.

## Example

```rust
let mut book = VecBook<SimpleOrder>::default();

book.add(SimpleOrder::sell(0, 2, 5));
book.add(SimpleOrder::sell(1, 3, 6));
book.add(SimpleOrder::sell(2, 4, 7));

book.remove(0);  // remove order with id 0

let mut fills = book.add(SimpleOrder::buy(3, 6, 6));

assert_eq!(fills, [Fill::full(1, 3, 6)]);
```

## Features

- allocation free during matching
- generic over the order type
- only supports limit orders. Market, IOC, ALO, etc. can be emulated on top.
