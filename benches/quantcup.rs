use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};
use lobster::{SimpleOrder, VecBook};

use lobster::OrderBook;
enum Action {
    Buy(SimpleOrder),
    Sell(SimpleOrder),
    Cancel(u32),
}

fn load_actions() -> Vec<Action> {
    let file = read_to_string("benches/orders.csv").expect("Failed to open file");
    let lines = file.lines().skip(1);

    let mut actions: Vec<Action> = Vec::with_capacity(35760);
    let mut order_id = 0;

    for line in lines {
        let mut fields = line.split(',');
        let _trader_id: u32 = fields.next().unwrap().parse().unwrap();
        let side: &str = fields.next().unwrap();
        let price: u32 = fields.next().unwrap().parse().unwrap();
        let quantity: u32 = fields.next().unwrap().parse().unwrap();

        if price == 0 {
            actions.push(Action::Cancel(quantity));
        } else if side == "Bid" {
            actions.push(Action::Buy(SimpleOrder::new(order_id, quantity, price)));
            order_id += 1;
        } else if side == "Ask" {
            actions.push(Action::Sell(SimpleOrder::new(order_id, quantity, price)));
            order_id += 1;
        } else {
            panic!("Invalid action: {}", side);
        }
    }
    actions
}

fn run_test(actions: &[Action]) {
    let mut book = VecBook::<SimpleOrder>::default();
    for action in actions {
        match *action {
            Action::Buy(order) => {
                book.buy(order).for_each(drop);
            }
            Action::Sell(order) => {
                book.sell(order).for_each(drop);
            }
            Action::Cancel(id) => {
                book.remove(id);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let actions = load_actions();
    c.bench_function("run test", |b| b.iter(|| run_test(&actions)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
