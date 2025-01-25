//! Order trait for orders in an order book.
use core::hash::Hash;
use core::ops::Sub;

/// An order in an order book.
pub trait Order: Clone {
    type OrderId: Eq + Hash + Clone;
    type Quantity: Copy + Ord + Sub<Output = Self::Quantity>;
    type Price: Ord;

    fn id(&self) -> Self::OrderId;
    fn quantity(&self) -> Self::Quantity;
    fn set_quantity(&mut self, quantity: Self::Quantity);

    #[expect(clippy::arithmetic_side_effects)]
    fn reduce_quantity(&mut self, quantity: Self::Quantity) {
        self.set_quantity(self.quantity() - quantity);
    }

    fn price(&self) -> Self::Price;
    /// Returns `true` if the order is a buy order, `false` if it is a sell order.
    fn is_buy(&self) -> bool;
}
