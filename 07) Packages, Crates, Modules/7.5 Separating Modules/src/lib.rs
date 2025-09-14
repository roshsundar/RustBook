/* Summary:
Rust lets you split a crate into modules so you can refer to items defined in one module from another module.
A module w/ a submodule can be defined in a separate file.
    - In the module.rs, declare the submodule
        - i.e. mod submodule;
    - Create a folder w/ submodule file in it, /module/submodule.rs

You can refer to module items by specifying absolute or relative paths. 
These paths can be brought into scope with a *use* statement,
so you can use a shorter path to use items in that scope. 

Module code is private by default, but you can make definitions public by adding the pub keyword.
*/

// Note: only need to load a file using mod once. After, other files in the project can refer to the file's code with paths.
// The body is in src/front_of_house.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}