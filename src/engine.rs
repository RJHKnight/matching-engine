use btreemultimap::BTreeMultiMap;
use crate::order_data::*;
use math::round::floor;
use std::collections::HashMap;

struct LitEngine {

    security_id: u32,
    bids: BTreeMultiMap<i32, OrderData>,
    asks: BTreeMultiMap<i32, OrderData>,
    market_state : MarketState,
    order_id: u64,
    id_to_price: HashMap<u64, i32>,
}

impl LitEngine {

    fn new(security_id: u32) -> LitEngine {
        
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

    fn to_int_price(price: f32) -> i32 {
        floor(price as f64 / 0.01, 0) as i32
    }

    pub fn create_order(&mut self, price: f32, quantity: u32, side: OrderSide, owner: i32, order_type: OrderType) -> u64 {
        
        let new_order_id = self.get_and_increment_id();
        self.create_order_int(price, quantity, side, owner, order_type, new_order_id)
    }


    fn create_order_int(&mut self, price: f32, quantity: u32, side: OrderSide, owner: i32, order_type: OrderType, order_id: u64) -> u64 {

        if order_type.is_dark() {
            panic!("Dark order sent to lit engine.")
        }

        let state = OrderData::new(price, quantity, owner, order_type, order_id);

        let book_side = if side == OrderSide::Buy { &mut self.bids} else {&mut self.asks};
    
        let int_price = Self::to_int_price(price);
        book_side.insert(int_price, state);

        // Update our mapping of id to price.
        self.id_to_price.insert(order_id, int_price);

        order_id
    }

    pub fn cancel_order(&mut self, id: u64, is_buy: bool) {

        let price = self.id_to_price.get(&id).unwrap();
        let book_side = if is_buy { &mut self.bids} else {&mut self.asks};

        let orders_at_price = book_side.get_vec_mut(price).unwrap();
        orders_at_price.retain(|o| !o.matches_id(id));

        // Clean up id mapping.
        self.id_to_price.remove(&id);
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
        let book_side = if is_buy { &mut self.bids} else {&mut self.asks};

        let orders_at_price = book_side.get_vec_mut(price).unwrap();
        let order = orders_at_price.iter_mut().find(|o| o.matches_id(id)).unwrap();

        // Need to cancel, new for upward quantity amend
        if order.is_amend_qty_up(new_quantity) {
            self.cancel_order(id, is_buy);
            self.create_order_int(order.price, order.quantity, order.)
        }
        else {
            order.amend_order_quantity(new_quantity);
        }
    }

    pub fn print_book(&self) {

    }

}

enum MarketState {
    PreOpen,
    Matching,
    Open,
    PreClose,
    Closed,
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

            for entry in  entries {
                let this_entry = entry.clone();
                assert!(this_entry > last_entry);
                println!("key: {}, this_entry: {}, last_entry: {}", this_key, this_entry, last_entry);

                last_entry = this_entry;
            }
        }
     }

    #[test]
    #[should_panic]
    fn invalid_price() {
        let order = OrderData::new(-1.0, 100, 1, OrderType::Limit, 1, OrderSide::Buy) ;
    }

    #[test]
    #[should_panic]
    fn invalid_qty() {
        let order = OrderData::new(1.0, 0, 1, OrderType::Limit, 1, OrderSide::Buy);
    }
}