mod engine;
use engine::engine::{MatchingEngine, TradingPair};
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

    // println!("{:?}", orederbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDT".to_string());
    engine.add_new_market(pair.clone());

    let eth_pair = TradingPair::new("ETH".to_string(), "USDT".to_string());

    // engine.add_new_market(eth_pair.clone());
    let buy_order = Order::new(BidOrAk::Bid, 45.5);
    let eth_order = Order::new(BidOrAk::Ask, 12.3);

    engine.place_limit_order(pair, 100.0, buy_order).unwrap();
    engine.place_limit_order(eth_pair, 23.4, eth_order).unwrap();
}
