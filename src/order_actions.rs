

// These are actions that a user can do to an order
#[derive(Debug)]
enum OrderAction {
    Cancel{},
    AmendPrice{price: f32},
    AmendQuantity{quantity: i32},
}

// This is the internal state that an order is in
enum OrderState {
    PendingNew,
    PendingAmendPrice,
    PendingAmendQuantity,
    Acked,
}

// This is an event that can happen to an order
enum OrderEvent {
    
    // New
    NewAck{},
    NewReject{reason: String},

    // Amend
    AmendAck{},
    AmendReject{reason: String},

    // Unsolicited
    Cancel{reason: String},
    PartialFill{qty: i32, price: f32, remaining: i32},
    FullFill{qty: i32, price: f32},
}