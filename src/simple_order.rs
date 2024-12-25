//! A simple implementation of the `Order` trait.
use crate::order::Order;

/// A simple order with a unique identifier, quantity, price, and whether it is a buy or sell order.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct SimpleOrder {
    /// The unique identifier of the order.
    order_id: u32,
    /// The quantity of the order.
    quantity: u32,
    /// The price of the order.
    price: u32,
    /// `true` if the order is a buy order, `false` if it is a sell order.
    is_buy: bool,
}

impl SimpleOrder {
    #[must_use]
    pub const fn buy(order_id: u32, quantity: u32, price: u32) -> Self {
        Self {
            order_id,
            quantity,
            price,
            is_buy: true,
        }
    }
    #[must_use]
    pub const fn sell(order_id: u32, quantity: u32, price: u32) -> Self {
        Self {
            order_id,
            quantity,
            price,
            is_buy: false,
        }
    }

    #[must_use]
    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.set_quantity(quantity);
        self
    }
}

impl Order for SimpleOrder {
    type OrderId = u32;
    type Quantity = u32;
    type Price = u32;

    fn id(&self) -> Self::OrderId {
        self.order_id
    }

    fn quantity(&self) -> Self::Quantity {
        self.quantity
    }

    fn set_quantity(&mut self, quantity: Self::Quantity) {
        self.quantity = quantity;
    }

    fn price(&self) -> Self::Price {
        self.price
    }

    fn is_buy(&self) -> bool {
        self.is_buy
    }
}
