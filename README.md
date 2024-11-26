# Orderbook

A simple limit orderbook in Rust.

- allocation free during matching
- matching returns iterator of fills
- only supports limit orders. Market / IOC orders, price changes, etc. can be emulated on top.