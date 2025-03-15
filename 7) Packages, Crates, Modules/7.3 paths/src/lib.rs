/*
A path is used to find an item in a module tree. There are 2 types:
    - absolute path = full path from crate root
        - for code in an external crate, starts w/ crate name
        - for code in the current craate, starts with crate::
    - relative path = starts in the current module. Uses self, super, or an identifier in the module. 
*/

mod front_of_house {
    // By default, items in modules are private. Use pub to make them visible outside of the module.

    // Making hosting public allows its name to be known to front_of_house.
    // However, the contents of hosting are default private.
    // pub on a module only lets ancestor modules refer to it, not access inner items. 
    pub mod hosting {
        // So to make add_to_waitlist() available to hosting, must make it pub.
        pub fn add_to_waitlist() {}
    }

    // Items in a parent module can't access private items in a child module.
    // However, items in child modules can use items in the ancestor modules.
}

pub fn eat_at_restaurant() {
    // Absolute path
    /* Trace the path, each step should be accessible from this function
    1. crate: root of the module tree
    2. front_of_house: not pub, but sibling with eat_at_restaurant() in the crate module
    3. hosting: pub accessible
    4. add_to_waitlist(): pub accessible
    */
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    /* Trace the path, each step should be accessible from this function
    1. front_of_house: not pub, but sibling with eat_at_restaurant() in the crate module
    2. hosting: pub accessible
    3. add_to_waitlist(): pub accessible
    */
    front_of_house::hosting::add_to_waitlist();

    /*
    In general, absolute paths are preferred, since it's more likely that
    modules will be defined separately from where they're used/called.
    */
}

/*
A guide to best Rust API practices: https://rust-lang.github.io/api-guidelines/

The best practices for a package with a binary & library crate:
    - treat the library (src/lib.rs) to have the functionality
        - module tree should be defined there
    - the binary crate should be treated like an external user of the library crate
*/

/*
$ Running *cargo modules structure* on the command line will print the module tree
crate restaurant
├── fn eat_at_restaurant: pub
└── mod front_of_house: pub(crate)
    └── mod hosting: pub
        └── fn add_to_waitlist: pub
*/

// Use *super* to construct relative paths that start in the parent module

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // deliver_order is defined in the parent module of back_of_house, crate in this case
        super::deliver_order();
    }

    fn cook_order() {}
}

/*
pub can designate structs and enums as public.

The fields of a struct are private by default, even if the struct definition is public.
Use pub on the struct field to make it public.

If an enum is public, all variants are public by default
*/

mod morning {
    pub struct Breakfast {
        pub toast: String, // toast is public and can be set externally
        seasonal_fruit: String, // seasonal_fruit is private, can't be set externally
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Drink {
        OJ,
    }
}

pub fn eat_in_morning() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = morning::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like a {} toast", meal.toast);

    // seasonal_fruit is private, so it can't be read or written to
    // meal.seasonal_fruit = String::from("blueberries"); //! err: field `seasonal_fruit` of `Breakfast` is private

    // The variants of a public enum are all avaliable
    let drink = morning::Drink::OJ;
}