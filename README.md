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
