use std::collections::{HashMap, VecDeque};

use crate::{fill::Fill, OrderId, Price, Quantity, MAX_PRICE};

pub struct OrderBook {
    order_qty: HashMap<OrderId, Quantity>,
    levels: [VecDeque<OrderId>; MAX_PRICE as usize + 1],
    bid: Price,
    ask: Price,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self {
            order_qty: HashMap::new(),
            levels: std::array::from_fn(|_| VecDeque::new()),
            bid: 0,
            ask: MAX_PRICE,
        }
    }
}

impl OrderBook {
    /// Returns the number of orders in the book.
    #[must_use]
    pub fn len(&self) -> usize {
        self.order_qty.len()
    }

    /// Returns `true` if the book contains no orders.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.order_qty.is_empty()
    }

    /// Adds a new buy order to the order book.
    pub fn buy(&mut self, id: OrderId, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        if quantity == 0 {
            return fills;
        }
        if self.bid != Price::MAX {
            while self.ask <= price {
                let level = &mut self.levels[usize::from(self.ask)];
                Self::level(
                    self.ask,
                    &mut quantity,
                    &mut self.order_qty,
                    level,
                    &mut fills,
                );
                if quantity == 0 {
                    return fills;
                }
                if self.ask == Price::MAX {
                    break;
                }
                self.ask += 1;
            }
        }
        self.bid = self.bid.max(price);
        self.levels[usize::from(price)].push_back(id);
        self.order_qty.insert(id, quantity);
        fills
    }

    /// Adds a new sell order to the order book.
    pub fn sell(&mut self, id: OrderId, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        if quantity == 0 {
            return fills;
        }
        if self.ask != Price::MIN {
            while price <= self.bid {
                let level = &mut self.levels[usize::from(self.bid)];
                Self::level(
                    self.bid,
                    &mut quantity,
                    &mut self.order_qty,
                    level,
                    &mut fills,
                );
                if quantity == 0 {
                    return fills;
                }
                if self.bid == Price::MIN {
                    break;
                }
                self.bid -= 1;
            }
        }
        self.ask = self.ask.min(price);
        self.levels[usize::from(price)].push_back(id);
        self.order_qty.insert(id, quantity);
        fills
    }

    /// Removes an order from the order book. Returns `false` if the order was
    /// not found.
    pub fn remove(&mut self, id: OrderId) -> bool {
        self.order_qty.remove(&id).is_some()
    }

    fn level(
        price: Price,
        qty: &mut Quantity,
        order_qty: &mut HashMap<OrderId, Quantity>,
        level: &mut VecDeque<OrderId>,
        fills: &mut Vec<Fill>,
    ) {
        let mut i = 0;
        for &maker_id in level.iter() {
            match order_qty.get_mut(&maker_id) {
                Some(maker_qty) => {
                    if *qty >= *maker_qty {
                        fills.push(Fill::new(maker_id, *maker_qty, price, true));
                        *qty -= *maker_qty;
                        order_qty.remove(&maker_id);
                        i += 1;
                    } else {
                        fills.push(Fill::new(maker_id, *qty, price, false));
                        *maker_qty -= *qty;
                        *qty = 0;
                        break;
                    }
                }
                None => i += 1,
            }
        }
        level.drain(..i); // theoretically O(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fill, OrderBook, Price, Quantity, MAX_PRICE};

    #[test]
    fn add_then_remove() {
        let mut book = OrderBook::default();
        book.buy(0, 4, 23);
        assert!(book.remove(0));
        assert!(!book.remove(0));
    }

    #[test]
    fn multiple_fills_with_cancel() {
        let mut book = OrderBook::default();
        book.sell(0, 2, 5);
        book.sell(1, 3, 6);
        book.sell(2, 4, 7);
        book.remove(0);
        let fills = book.buy(3, 6, 7);

        assert_eq!(
            fills,
            vec![Fill::new(1, 3, 6, true), Fill::new(2, 3, 7, false)]
        );
    }

    #[test]
    fn fire_for_order_that_was_filled_exactly() {
        let mut book = OrderBook::default();
        book.sell(0, 2, 23);
        let fills = book.buy(1, 2, 23);
        assert_eq!(fills, vec![Fill::new(0, 2, 23, true)]);
        let fills = book.buy(2, 2, 23);
        assert_eq!(fills, vec![]);

        let mut book = OrderBook::default();
        book.buy(0, 2, 23);
        let fills = book.sell(1, 2, 23);
        assert_eq!(fills, vec![Fill::new(0, 2, 23, true)]);
        let fills = book.sell(2, 2, 23);
        assert_eq!(fills, vec![]);
    }

    #[test]
    fn fire_for_order_that_was_filled_excessively() {
        let mut book = OrderBook::default();
        book.sell(0, 1, 23);
        let fills = book.buy(1, 2, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, true)]);
        let fills = book.buy(2, 1, 23);
        assert_eq!(fills, vec![]);

        let mut book = OrderBook::default();
        book.buy(0, 1, 23);
        let fills = book.sell(1, 2, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, true)]);
        let fills = book.sell(2, 1, 23);
        assert_eq!(fills, vec![]);
    }

    #[test]
    fn trade_twice_with_resting_order() {
        let mut book = OrderBook::default();
        book.sell(0, 2, 23);
        let fills = book.buy(1, 1, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, false)]);
        let fills = book.buy(2, 1, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, true)]);

        let mut book = OrderBook::default();
        book.buy(0, 2, 23);
        let fills = book.sell(1, 1, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, false)]);
        let fills = book.sell(2, 1, 23);
        assert_eq!(fills, vec![Fill::new(0, 1, 23, true)]);
    }

    #[test]
    fn test_quantity_limits() {
        let mut book = OrderBook::default();
        book.buy(0, Quantity::MAX, 23);
        let fills = book.sell(1, Quantity::MAX, 23);
        assert_eq!(fills, vec![Fill::new(0, Quantity::MAX, 23, true)]);

        let mut book = OrderBook::default();
        book.sell(0, Quantity::MAX, 23);
        let fills = book.buy(1, Quantity::MAX, 23);
        assert_eq!(fills, vec![Fill::new(0, Quantity::MAX, 23, true)]);
    }

    #[test]
    fn trade_twice_with_resting_order_price_limits() {
        let mut book = OrderBook::default();
        book.sell(0, 2, Price::MIN);
        let fills = book.buy(1, 1, Price::MIN);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MIN, false)]);
        let fills = book.buy(2, 1, Price::MIN);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MIN, true)]);

        let mut book = OrderBook::default();
        book.buy(0, 2, Price::MIN);
        let fills = book.sell(1, 1, Price::MIN);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MIN, false)]);
        let fills = book.sell(2, 1, Price::MIN);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MIN, true)]);

        let mut book = OrderBook::default();
        book.sell(0, 2, Price::MAX);
        let fills = book.buy(1, 1, Price::MAX);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MAX, false)]);
        let fills = book.buy(2, 1, Price::MAX);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MAX, true)]);

        let mut book = OrderBook::default();
        book.buy(0, 2, Price::MAX);
        let fills = book.sell(1, 1, Price::MAX);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MAX, false)]);
        let fills = book.sell(2, 1, Price::MAX);
        assert_eq!(fills, vec![Fill::new(0, 1, Price::MAX, true)]);
    }
}
