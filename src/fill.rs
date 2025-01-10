//! A fill represents an execution.

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Fill<OrderType> {
    Partial(OrderType),
    Full(OrderType),
}

impl<OrderType> Fill<OrderType> {
    pub const fn as_ref(&self) -> &OrderType {
        match self {
            &Self::Partial(ref order) | &Self::Full(ref order) => order,
        }
    }

    pub fn unwrap(self) -> OrderType {
        match self {
            Self::Partial(order) | Self::Full(order) => order,
        }
    }
}
