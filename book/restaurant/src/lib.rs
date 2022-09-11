mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn place_at_table() {}

    }
    pub mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_housse {
    fn fix_incorrect_order(){
        super::front_of_house::serving::take_order();    
        super::serve_order();
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
}

fn serve_order() {}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // starting reference is the module defined at the same level in the module tree as the calling function
    front_of_house::hosting::add_to_waitlist();
}
