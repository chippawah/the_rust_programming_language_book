mod back_of_house;
mod front_of_house;

fn deliver_order() {}
pub mod customer {
    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist(); // using a full path here with the "crate" literal

        use crate::{back_of_house, front_of_house}; // the use keyword can bring paths into scope
        front_of_house::hosting::seat_at_table(); // using the module by name here as it's in scopee
        front_of_house::serving::take_order();

        let mut order = back_of_house::Breaktfast::summer(String::from("Rye"));
        order.toast = String::from("Wheat");

        let _appetizer = back_of_house::Appetizer::Salad;
        let _appetizer = back_of_house::Appetizer::Soup;

        back_of_house::cook_order();
        front_of_house::serving::serve_order();
        front_of_house::serving::take_payment();
    }
}
