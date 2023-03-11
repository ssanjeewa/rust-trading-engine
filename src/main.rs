mod engine;
use engine::orderbook::{BidOrAk, Order, Orderbook};

fn main() {
    let buy_order = Order::new(BidOrAk::Bid, 5.5);
    let buy_order_bob = Order::new(BidOrAk::Bid, 3.5);

    let sell_order = Order::new(BidOrAk::Ask, 5.5);
    let sell_order_bob = Order::new(BidOrAk::Ask, 3.5);

    let mut orederbook = Orderbook::new();
    orederbook.add_order(5.5, buy_order_bob);
    orederbook.add_order(5.5, buy_order);

    orederbook.add_order(6.0, sell_order);
    orederbook.add_order(6.0, sell_order_bob);

    println!("{:?}", orederbook);
}
