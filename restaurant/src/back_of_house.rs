use crate::front_of_house::serving;

fn fix_incorrect_order() {
    cook_order();
    serving::serve_order();
}

fn cook_order() {}
mod cooking {
    fn prepare_dish() {}
    fn send_dish_to_front() {}
}

mod dishwashing {
    fn wash_dishes() {}
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
