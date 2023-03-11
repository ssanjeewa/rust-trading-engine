use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }
    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAk::Bid => {
                let price = Price::new(price);

                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAk::Ask => {}
        }
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAk,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        Price {
            integral,
            fractional,
            scalar,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

impl Order {
    fn new(bid_or_ask: BidOrAk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}

fn main() {
    let buy_order = Order::new(BidOrAk::Bid, 5.5);
    let buy_order_bob = Order::new(BidOrAk::Bid, 3.5);

    let mut orederbook = Orderbook::new();
    orederbook.add_order(5.5, buy_order_bob);
    orederbook.add_order(5.5, buy_order);

    println!("{:?}", orederbook);
}
