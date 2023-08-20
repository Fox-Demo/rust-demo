//note 這邊整個是 kata 模組

pub mod enums;
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
    use inner::innerFn;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    pub fn sibling() {
        innerFn();
    }
}
