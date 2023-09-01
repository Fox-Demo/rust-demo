#[allow(dead_code)]
//note 這邊整個是 kata 模組
pub mod collections;
pub mod enums;
pub mod error;
pub mod generic;
pub mod structs;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {

    mod inner {
        pub fn innerFn() {}
    }

    //note 因為採用絕對路徑，所以要從 crate -> kata -> front_of_house -> hosting 找下去
    //use crate::kata::front_of_house::hosting;

    //note 這邊使用相對路徑，所以需要先用 super 回到 kata module
    use super::front_of_house::hosting;

    //note inner module 也一樣在 customer 裡面，因此如果要用 inner 不用 super

    use inner::innerFn;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    pub fn sibling() {
        innerFn();
    }
}
