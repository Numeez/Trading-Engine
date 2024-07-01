use std::collections::HashMap;

#[derive(Debug)]
enum BidAsk{
    Bid,
    Ask,
}

#[derive(Debug)]
struct Orderbook {
    asks : HashMap<Price,Limit>,
    bids :  HashMap<Price,Limit>,
}

impl Orderbook {
    fn new()->Orderbook{
        Orderbook { 
            asks: HashMap::new(), 
            bids: HashMap::new(),
         }
    }
    fn add_limit_order(&mut self ,price : f64, order:Order){
        match order.bid_or_ask {
        BidAsk::Bid =>{
            let price = Price::new(price);



            match  self.bids.get_mut(&price) {
                Some(limit)=>println!("Already got a limit !"),
                None=>{
            
                }
            }
        }
        BidAsk::Ask=>{

        }
            
        }
    }
}

#[derive(Debug,PartialEq,Eq,Hash)]
struct  Price {
    integral : u64,
    fractional:u64,
    scalar:u64,
}

impl  Price {
    fn new(price:f64)->Price{
        let scalar=100000;
        let integral=price as u64;
        let fractional =  ((price%0.1) * scalar as f64) as u64;
        Price { integral,  fractional,scalar }
    }
   
}
#[derive(Debug)]
struct Limit{
    price : Price,
    orders: Vec<Order>
}
impl Limit {
    fn new(price: f64)->Limit{
        Limit { 
            price: Price::new(price),
             orders: Vec::new() ,
            }
    }
    fn add_order(&mut self,order:Order){
        self.orders.push(order)

    }
}
#[derive(Debug)]
struct  Order {
    size: f64,
    bid_or_ask:BidAsk

}

impl Order {
    fn new(bid_or_ask : BidAsk,size : f64)->Order{
        Order { size: size, bid_or_ask: bid_or_ask }
    }
}


fn main() {
    let mut limit = Limit::new(65.5);
 
    let buy_order = Order::new(BidAsk::Bid, 12.2);
    let sell_order = Order::new(BidAsk::Ask, 2.2);
   
    let mut orderbook  = Orderbook::new();
    orderbook.add_limit_order(44.5, buy_order);
   
    println!("{:?}",orderbook);
}
