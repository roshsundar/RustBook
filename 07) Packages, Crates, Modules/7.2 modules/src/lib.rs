/* Summary:
Modules let us organize code within a crate for readability and easy reuse.

Modules also allow us to control the privacy of items because code within a module is private by default. 
Private items are internal implementation details not available for outside use. 
We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
*/

// Module declared with mod
mod front_of_house {
    // Modules can be nested
    mod hosting {
        fn _add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        // Body of module can hold definitions for items - i.e. structs, enums, constants, traits, functions

        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}
/* A module tree shows the hierarchy of modules. The module tree for the above code is:
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Modules have parent-child relationships. I.e. front_of_house is a child of crate, and a parent of serving.

hosting and serving are siblings, meaning they're defined in the same module (front_of_house).

crate is an implicit module.
*/