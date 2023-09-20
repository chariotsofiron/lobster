use std::collections::{HashMap, VecDeque};

use crate::{fill::Fill, Price, Quantity, MAX_PRICE};

pub struct OrderBook {
    order_qty: HashMap<usize, Quantity>,
    levels: [VecDeque<usize>; MAX_PRICE as usize + 1],
    best_bid: Price,
    best_ask: Price,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self {
            order_qty: HashMap::new(),
            levels: std::array::from_fn(|_| VecDeque::new()),
            best_bid: 0,
            best_ask: MAX_PRICE,
        }
    }
}

impl OrderBook {
    /// Returns the number of resting orders in the book.
    #[must_use]
    pub fn len(&self) -> usize {
        self.order_qty.len()
    }

    /// Returns `true` if the book is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.order_qty.is_empty()
    }

    /// Adds a new buy order to the order book.
    pub fn buy(&mut self, id: usize, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        for px in self.best_ask..=price {
            self.best_ask = px;
            let level = &mut self.levels[usize::from(px)];
            Self::level(px, &mut quantity, &mut self.order_qty, level, &mut fills);
            if quantity == 0 {
                return fills;
            }
        }
        self.best_bid = self.best_bid.max(price);
        self.levels[usize::from(price)].push_back(id);
        self.order_qty.insert(id, quantity);
        fills
    }

    /// Adds a new sell order to the order book.
    pub fn sell(&mut self, id: usize, mut quantity: Quantity, price: Price) -> Vec<Fill> {
        let mut fills = Vec::new();
        for px in (price..=self.best_bid).rev() {
            self.best_bid = px;
            let level = &mut self.levels[usize::from(px)];
            Self::level(px, &mut quantity, &mut self.order_qty, level, &mut fills);
            if quantity == 0 {
                return fills;
            }
        }
        self.best_ask = self.best_ask.min(price);
        self.levels[usize::from(price)].push_back(id);
        self.order_qty.insert(id, quantity);
        fills
    }

    /// Removes an order from the order book. Returns `false` if the order was
    /// not found.
    pub fn remove(&mut self, id: usize) -> bool {
        self.order_qty.remove(&id).is_some()
    }

    fn level(
        price: Price,
        qty: &mut Quantity,
        order_qty: &mut HashMap<usize, Quantity>,
        level: &mut VecDeque<usize>,
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
        level.drain(..i);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Fill, OrderBook, Quantity, MAX_PRICE};

    #[test]
    fn test_add_then_remove() {
        let mut book = OrderBook::default();
        book.buy(0, 4, 20);
        assert!(book.remove(0));
        assert!(!book.remove(0));
    }

    #[test]
    fn test_multiple_fills_with_cancel() {
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
    fn test_trade_max_qty() {
        let mut book = OrderBook::default();
        book.sell(0, Quantity::MAX, MAX_PRICE);
        let fills = book.buy(1, Quantity::MAX, MAX_PRICE);
        assert_eq!(fills, vec![Fill::new(0, Quantity::MAX, MAX_PRICE, true)]);
    }
}
