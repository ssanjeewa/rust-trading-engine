# Orderbook Data Structure in Rust

The `Orderbook` data structure is used for financial exchange and stores Limit orders at particular prices using a hashmap. The given Rust code defines the implementation of the `Orderbook` data structure.

## Implementation

The code contains the following structs and enums:

`BidOrAk` enum
An enum with two variants: `Bid` and `Ask`. These represent the two types of orders in a financial exchange.

## `Order` struct

A struct representing an order in the exchange. It contains two fields:

    `size`: The size of the order
    `bid_or_as`k: The type of order, either `Bid` or `Ask`.

## `Price` struct

A struct representing the price of an order. It contains three fields:

    `integral`: The integer part of the price
    `fractional`: The fractional part of the price
    `scalar`: A scalar value used to convert the price to integer value for the purposes of the data structure.

## `Limit` struct

A struct representing a Limit order at a particular price. It contains two fields:

    `price`: The price of the limit order
    `orders`: A vector containing all orders at the particular price.

## `Orderbook` struct

A struct representing the `Orderbook` data structure. It contains two fields:

    `bids`: A hashmap of all `Bid` Limit orders, indexed by price.
    `asks`: A hashmap of all `Ask` Limit orders, indexed by price.

The `Orderbook` struct has two methods:

    `new()`: Creates a new Orderbook instance with empty bids and asks hashmaps.

    `add_order()`: Adds a new order to the orderbook. It takes two arguments:
    	`price`: The price of the order.
    	`order`: The order itself, containing the size and bid_or_ask fields.

If the order is a `Bid` order, it is added to the `bids` hashmap at the corresponding price. If the price already has a `Limit` order, the new order is added to its vector of orders. If not, a new `Limit` instance is created and added to the hashmap.

If the order is an `Ask` order, it is not added to the orderbook. The code currently contains no implementation for handling `Ask` orders.

# Example Usage

```rust
	let mut orderbook = Orderbook::new();
	let bid_order = Order::new(BidOrAk::Bid, 10.0);
	let price = 100.0;
	orderbook.add_order(price, bid_order);
	println!("{:?}", orderbook.bids);
```

This will create a new `Orderbook` instance, create a `Bid` order with size 10.0, and add it to the `Orderbook` at a price of 100.0. The hashmap of bids in the `orderbook` will then be printed to the console.

# Matching Engine

The Matching Engine is a Rust library that provides a basic implementation of a matching engine for a cryptocurrency exchange. It includes functionality for creating and managing orderbooks for different trading pairs and placing limit orders in those orderbooks.

## TradingPair

The `TradingPair` struct represents a pair of base and quote currencies. It has two fields, `base` and `quote`, which are both of type `String`.

The `TradingPair` struct has two methods defined on it:

    `new(base: String, quote: String) -> TradingPair`: Creates a new `TradingPair` instance with the provided base and quote fields.

    `to_string(self) -> String:` Returns a formatted string representation of the `TradingPair` instance in the format "base_quote".

## Order

The `Order` struct represents an order for a particular trading pair. It has two fields, `quantity` and `side`, which are both of type `f64`.

## MatchingEngine

The `MatchingEngine` struct manages `orderbooks` for different trading pairs. It has a single field, `orderbooks`, which is a `HashMap` that maps `TradingPair` instances to `Orderbook` instances.

The `MatchingEngine` struct has two methods defined on it:

    `new() -> MatchingEngine`: Creates a new `MatchingEngine` instance with an empty `orderbooks` `HashMap`.

    `add_new_market(&mut self, pair: TradingPair):` Adds a new entry to the `orderbooks` `HashMap` with the `TradingPair` as the key and a new `Orderbook` instance as the value.

    `place_limit_order(&mut self, pair: TradingPair, price: f64, order: Order) -> Result<(), String>:` Adds the provided Order instance to the `Orderbook` for the specified `TradingPair` at the specified price level.

## Examples

Creating a new `TradingPair` instance:

```rust
let pair = TradingPair::new(String::from("BTC"), String::from("USD"));

```

Adding a new `TradingPair` and `Order` instance to the `MatchingEngine`:

```rust
let mut engine = MatchingEngine::new();
let pair = TradingPair::new(String::from("BTC"), String::from("USD"));
let order = Order { quantity: 1.0, side: String::from("buy") };
engine.add_new_market(pair.clone());
engine.place_limit_order(pair.clone(), 10000.0, order.clone());

```
