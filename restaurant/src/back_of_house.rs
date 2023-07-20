pub enum Appetizer {
    Soup,
    Salad,
}
pub struct Breaktfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breaktfast {
    pub fn summer(toast: String) -> Breaktfast {
        Breaktfast {
            toast,
            seasonal_fruit: String::from("peach"),
        }
    }
}

fn fix_order() {
    cook_order();
    super::deliver_order();
}
pub fn cook_order() {}
