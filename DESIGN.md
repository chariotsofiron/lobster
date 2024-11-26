# Design

Some notes about the design.


## Why have `buy` and `sell` methods instead of `add` with a `side` parameter?

- Don't want to export custom Side enum that user likely already has
- Avoids a branch in the code
- order type would need a side field, increasing its size
- orderbook implementations often store bids and asks separately, and side can be implicitly derived

## Why not have an `on_update(op: Op)` instead of buy, sell, cancel methods?

- what would the return type be? Calling respective functions has context of what to expect.
- avoids conditional branching
- simplifies the function signature


## Why not use signed quantity to represent sells?

- Code becomes more annoying to reason about
- Panics on `Quantity::MIN.neg()`
- What would the sign of Fill's quantity be?
- con: quantity needs to be casted when updating position (likely signed)

## Why not have the order book generate the order ID?

- tuple return is yucky
- introduces unnecessary mapping, constant conversions
- Order IDs should be unique across all order books

## Why is order modify the way it is?

- Setting the quantity to a value avoids subtraction / underflow
- Setting the price to zero would imply cancelling it, which needs to be done via remove

## Why have `done` field on `Fill`?

- TBD