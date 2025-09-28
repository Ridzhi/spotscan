fn main() {
    println!("Hello, world!");
}

pub mod spot {
    pub struct Slot {
        pub time: SlotTime,
        pub plgr_1: Plgr,
        pub plgr_2: Plgr,
        pub plgr_3: Plgr,
        pub plgr_4: Plgr,
        pub plgr_5: Plgr,
    }

    pub struct Plgr {
        pub price: u16,
        pub free: bool,
        pub group: Option<PlgrGroup>,
    }

    pub struct PlgrGroup {
        pub id: String,
        pub time: Option<SlotTime>,
        pub slots_count: u8,
    }

    pub struct SlotTime(pub String);
}
