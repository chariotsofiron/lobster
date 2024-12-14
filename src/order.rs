//! Order trait for orders in an order book.
use core::ops::Sub;

/// An order in an order book.
pub trait Order {
    type OrderId: Eq;
    type Quantity: Copy + Ord + Default + Sub<Output = Self::Quantity>;
    type Price: Copy + Ord;

    fn id(&self) -> Self::OrderId;
    fn quantity(&self) -> Self::Quantity;
    fn set_quantity(&mut self, quantity: Self::Quantity);
    fn price(&self) -> Self::Price;
    /// Returns true if the order is a buy order, false if it is a sell order.
    fn is_buy(&self) -> bool;
}
