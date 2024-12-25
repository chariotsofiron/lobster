# Design

Some notes about the design.

## Should `Order` have `is_buy` field?

pros

- simplifies api. One method add instead of separate buy and sell
    - avoids `match side { Buy => self.buy(order), Sell => self.sell(order) }`
- one return type instead of potentially two
- some book implementations may benefit from having the side field
- can implement `OrderBook::from_iter`

cons

- order type would need a side field, increasing its size. Isn't necessary in many orderbook implementations
- orderbook implementations often store bids and asks separately, and side can be implicitly derived
- separate buy and sell methods with different return types can avoid branching
    - don't need initial buy/sell check
    - may need to check side flag on match iteration
    - branch prediction may make all these irrrelevant

## Why return iterator?

There are 3 options

1. `fn add(&mut self, order: Order, &mut Vec<Fill>) -> ()`
2. `fn add(&mut self, order: Order) -> Vec<Fill>`
3. `fn add(&mut self, order: Order) -> impl Iterator<Item=Fill>`

- First is prone to misuse if caller doesn't clear vector.
- Second requires allocations for every add
- Third avoids allocations, allows interrupting order matching if needed. Puts control in the hands of the caller.


## Why not use signed quantity to represent sells?

- Code becomes more annoying to reason about
- `Quantity::MIN.neg()` would panic
- What would the sign of Fill's quantity be?
- con: quantity needs to be casted when updating position (likely signed)

## Why not have the order book generate the order ID?

- tuple return is yucky
- introduces unnecessary mapping, constant conversions
- Order IDs should be unique across all order books

## Why is order modify the way it is?

- Setting the quantity to a value avoids subtraction / underflow
- Setting the price to zero would imply cancelling it, which needs to be done via remove

## Why no market order or other order types?

- Other order types can be emulated using limit orders

## Why no mid price function?

- price is generic and mid price may not be well-defined


## Order reference

Returning order references would be nice
how do we indicate fills?
update quantity to represent quantity filled
how do we indicate whether order was completely filled?