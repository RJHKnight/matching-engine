pub struct L1QuoteShapShot {
    
    pub bid_price: f32,
    pub bid_quantity: u32,
    pub bid_number: u32,

    pub ask_price: f32,
    pub ask_quantity: u32,
    pub ask_number: u32,
}

pub struct L2QuoteSnapShot {

    pub bid_price: Vec<f32>,
    pub bid_quantity: Vec<u32>,
    pub bid_number: Vec<u32>, // Num of orders at this level

    pub ask_price: Vec<f32>,
    pub ask_quantity: Vec<u32>,
    pub ask_number: Vec<u32>, // Num of orders at this level
}

pub struct L2QuoteDelta {

    pub price: f32,
    pub quantity: u32,
    pub number: u32,
    pub is_buy: bool,
}

pub struct AuctionData {
    
    pub match_price: f32,
    pub match_quantity: u32,
    pub imbalance: i32,
}


