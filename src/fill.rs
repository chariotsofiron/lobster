use crate::{Price, Quantity};


pub struct Fill {
    pub id: usize,
    pub quantity: Quantity,
    pub price: Price,
    pub done: bool,
}