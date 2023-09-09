# Design

Some notes about the design.


## Why have `buy` and `sell` methods instead of `add` with a `side` parameter?

- Don't want to export custom Side enum that user would already have
- Avoids a branch in the code

## Why not use signed quantity to represent sells?

- Code becomes more annoying to reason about
- Panics on `Quantity::MIN.neg()`
- What should the sign of Fill's quantity be

## Why not have the order book generate the order ID?

- tuple return is yucky
- introduces unnecessary mapping, constant conversions
- Order IDs should be unique across all order books

## Why have `done` field on `Fill`?

- TBD