use btreemultimap::BTreeMultiMap;
use crate::order_data::*;
use math::round::floor;
use std::collections::HashMap;
use std::cmp;

struct LitEngine {

    security_id: u32,
    bids: BTreeMultiMap<i32, OrderData>,
    asks: BTreeMultiMap<i32, OrderData>,
    market_state : MarketState,
    order_id: u64,
    id_to_price: HashMap<u64, i32>,
}

impl LitEngine {

    pub fn new(security_id: u32) -> LitEngine {
        
        LitEngine {
            security_id: security_id,
            bids: BTreeMultiMap::new(),
            asks: BTreeMultiMap::new(),
            market_state: MarketState::PreOpen,
            order_id: 0,
            id_to_price: HashMap::new(),
        }
    }

    fn get_and_increment_id(&mut self) -> u64 {
        let cur_id = self.order_id;
        self.order_id += 1;
        cur_id
    }

    pub fn create_order(&mut self, price: f32, quantity: u32, side: OrderSide, owner: i32, order_type: OrderType) -> u64 {
        
        let new_order_id = self.get_and_increment_id();
        let book_side = if side == OrderSide::Buy { &mut self.bids} else {&mut self.asks};
        create_order_int(book_side, &mut self.id_to_price, price, quantity, owner, order_type, new_order_id)
    }

    pub fn cancel_order(&mut self, id: u64, is_buy: bool) {

        let book_side = if is_buy { &mut self.bids} else {&mut self.asks};
        cancel_order_int(book_side, &mut self.id_to_price, id)
    }

    pub fn amend_order_price(&mut self, id: u64, new_price: f32, is_buy: bool) {
        
        let old_price = self.id_to_price.get(&id).unwrap();
        let book_side = if is_buy { &mut self.bids} else {&mut self.asks};

        let orders_at_price = book_side.get_vec_mut(old_price).unwrap();
        let order = orders_at_price.iter_mut().find(|o| o.matches_id(id)).unwrap();
        order.amend_order_price(new_price);
    }

    pub fn amend_order_quantity(&mut self, id: u64, new_quantity: u32, is_buy: bool) {
        
        let price = self.id_to_price.get(&id).unwrap();
        let book_side = if is_buy {&mut self.bids} else {&mut self.asks};
        let orders_at_price = book_side.get_vec(price).unwrap();
        let order = orders_at_price.iter().find(|o| o.matches_id(id)).unwrap();

        // Need to cancel, new for upward quantity amend
        if order.is_amend_qty_up(new_quantity) {

            create_order_int(book_side, &mut self.id_to_price, order.price, order.quantity, order.owner, order.order_type, order.id);
            cancel_order_int(book_side, &mut self.id_to_price, id);
            
        }
        else {
            let orders_at_price = book_side.get_vec_mut(price).unwrap();
            let order = orders_at_price.iter_mut().find(|o| o.matches_id(id)).unwrap();
            order.amend_order_quantity(new_quantity);
        }
    }

    pub fn check(&self) -> Option<Vec<Trade>> {

        match self.market_state {
            
            MarketState::Closed => None,
            MarketState::PreClose => None,
            MarketState::PreOpen => None,
            MarketState::Open => {
                
                if !self.is_overlapping() {
                    return Option::None;
                }
                
                // Overlapping book - time for some trading.
                let mut trades = Vec::new();

                while self.is_overlapping() {
    
                    trades.push(self.get_trade());

                    // trades.push(Trade{price: 10.0, qty: 10, is_full: true, buy_owner: 1, sell_owner: 1, trade_type: TradeType::AskInitiated });
                }

                
                
                Some(trades)
            },
            MarketState::Matching => None,
        }

    }

    fn is_overlapping(&self) -> bool {
        self.bids.iter().next().unwrap().0 >= self.asks.iter().next().unwrap().0
    }

    fn get_trade(&mut self) -> Trade {

        let buy_order = self.bids.iter_mut().next().unwrap().1;
        let sell_order = self.asks.iter_mut().next().unwrap().1;

        let trade_size = cmp::min(buy_order.quantity, sell_order.quantity);

        if !(buy_order.price == sell_order.price) {
            panic!("Matching with invalid prices! - buy: {}, sell: {}", buy_order.price, sell_order.price)
        }

        let trade = Trade{
             price: buy_order.price,
             qty: trade_size,
             buy_owner: buy_order.owner,
             sell_owner: sell_order.owner,
             trade_type: TradeType::MidPoint
        };

        // Adjust orders - at least one side is always fully filled
        // if buy_order.quantity == trade_size {
        //     cancel_order_int(&mut self.bids, &mut self.id_to_price, buy_order.id);
        // } 
        // if sell_order.quantity == trade_size {
        //     cancel_order_int(&mut self.asks, &mut self.id_to_price, sell_order.id);
        // }

        // if buy_order.quantity != sell_order.quantity {
        //     let amend_buy = buy_order.quantity > sell_order.quantity;
        //     let order_to_amend = if amend_buy { buy_order } else { sell_order };
        //     let new_qty = order_to_amend.quantity - trade_size;
        //     self.amend_order_quantity(order_to_amend.id, new_qty, amend_buy);
        // } 

        trade
        }
        
        pub fn print_book(&self) {
    
        }

     }


fn cancel_order_int(book_side : &mut BTreeMultiMap<i32, OrderData>, id_to_price: &mut HashMap<u64, i32>, id: u64) {
        
    let price = id_to_price.get(&id).unwrap();
    let orders_at_price = book_side.get_vec_mut(price).unwrap();
    orders_at_price.retain(|o| !o.matches_id(id));

    // Clean up id mapping.
    id_to_price.remove(&id);
}

fn create_order_int(book_side : &mut BTreeMultiMap<i32, OrderData>, id_to_price: &mut HashMap<u64, i32>, 
    price: f32, quantity: u32, owner: i32, order_type: OrderType, order_id: u64) -> u64 {

    if order_type.is_dark() {
        panic!("Dark order sent to lit engine.")
    }

    let state = OrderData::new(price, quantity, owner, order_type, order_id);

    let int_price = to_int_price(price);
    book_side.insert(int_price, state);

    // Update our mapping of id to price.
    id_to_price.insert(order_id, int_price);

    order_id
}

fn to_int_price(price: f32) -> i32 {
    floor(price as f64 / 0.01, 0) as i32
}

enum MarketState {
    PreOpen,
    Matching,
    Open,
    PreClose,
    Closed,
}

#[derive(PartialEq)]
pub enum OrderSide {
    Buy,
    Sell{is_short: bool},
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    // Test that the map maintains the ordering of the entries.
    #[test]
    fn test_btree_multimap() {
 
        let mut multimap = BTreeMultiMap::new();
        let mut rng = rand::thread_rng();

        // Populate map.
        let mut i = 1;

        while i < 100 {

            multimap.insert(rng.gen_range(0..4), i);
            i = i+1;
        }

        println!("len: {}", multimap.len());

        let mut last_key = -1;

        for key in multimap.keys() {

            // Check keys are sorted in ascending order.
            let this_key = key.clone();
            assert!(this_key > last_key);
            last_key = this_key;

            // Check order of multimap entries
            let entries = multimap.get_vec(key).unwrap();
            let mut last_entry = -1;

            for entry in entries {
                let this_entry = entry.clone();
                assert!(this_entry > last_entry);
                println!("key: {}, this_entry: {}, last_entry: {}", this_key, this_entry, last_entry);

                last_entry = this_entry;
            }
        }
     }
     
     #[test]
     fn test_lit_engine() {
        let mut engine = LitEngine::new(1);

        // Place some orders
        engine.create_order(10.0, 100, OrderSide::Buy, 1, OrderType::Limit);
        engine.create_order(10.0, 200, OrderSide::Buy, 2, OrderType::Limit);
        engine.create_order(10.0, 300, OrderSide::Buy, 3, OrderType::Limit);
        engine.create_order(10.0, 400, OrderSide::Buy, 4, OrderType::Limit);
        engine.create_order(9.0, 500, OrderSide::Buy, 5, OrderType::Limit);
        
        engine.create_order(11.0, 100, OrderSide::Sell { is_short: false }, 6, OrderType::Limit);
        engine.create_order(11.0, 200, OrderSide::Sell { is_short: false }, 7, OrderType::Limit);
        engine.create_order(11.0, 300, OrderSide::Sell { is_short: false }, 8, OrderType::Limit);
        engine.create_order(11.0, 400, OrderSide::Sell { is_short: false }, 9, OrderType::Limit);
        engine.create_order(12.0, 500, OrderSide::Sell { is_short: false }, 10, OrderType::Limit);

        
     }


}