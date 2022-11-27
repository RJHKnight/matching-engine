pub struct OrderData {
    pub price: f32,
    pub quantity: u32,
    pub owner: i32,
    pub order_type: OrderType,
    pub id: u64,
    pub order_side: OrderSide,
}

#[derive(Debug)]
pub enum OrderType {
    Limit,
    Market,
    Dark,
}

#[derive(PartialEq)]
enum OrderSide {
    Buy,
    Sell{is_short: bool},
}

impl OrderType {

    pub fn is_dark(&self) -> bool {
        match *self { 
            OrderType::Dark => true, 
            _ => false, 
        }
    }

    pub fn is_market(&self) -> bool {
        match *self { 
            OrderType::Market => true, 
            _ => false, 
        }
    }
}

impl OrderData {

    pub fn new(price: f32, quantity: u32, owner: i32, order_type: OrderType, id: u64, order_side: OrderSide) -> OrderData {

        if quantity <= 0 {
            panic!("Invalid  quantity")
        } 
        else if !order_type.is_market() && price <= 0.0 {
            panic!("Invalid  price")
        }
        else {
            OrderData {
                price,
                quantity,
                owner,
                order_type,
                id,
                order_side
            }
        }
    }

    fn is_dark(&self) -> bool {
        self.order_type.is_dark()
    }

    fn is_market(&self) -> bool {
        self.order_type.is_market()
    }

    pub fn matches_id(&self, id: u64) -> bool {
        self.id == id
    }

    pub fn amend_order_price(&mut self, new_price :f32) {
        self.price = new_price;
    }

    pub fn amend_order_quantity(&mut self, new_quantity: u32) {
        self.quantity = new_quantity;
    }

    pub fn is_amend_qty_up(&mut self, new_quantity: u32) -> bool {
        self.quantity > new_quantity
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn order_construction() {
        let mut order = OrderData::new(10.0, 1000, 1, OrderType::Limit, 1, OrderSide::Buy);
        assert_eq!(order.price,10.0);

        order = OrderData::new(11.0, 100, 10, OrderType::Dark, 1, OrderSide::Buy);
        assert_eq!(order.price, 11.0);
        assert!(order.is_dark());

        order = OrderData::new(11.0, 100, 10, OrderType::Market, 1, OrderSide::Buy);
        assert_eq!(order.price, 11.0);
        assert!(order.is_market());
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