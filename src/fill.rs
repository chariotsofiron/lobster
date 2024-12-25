//! A fill represents an execution.
use crate::order::Order;

#[derive(Debug, PartialEq, Eq)]
pub enum Fill<OrderType: Order> {
    Partial(OrderType),
    Full(OrderType),
}
