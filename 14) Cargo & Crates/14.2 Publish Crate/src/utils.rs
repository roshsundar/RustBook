// pub use shortens the path to use useful_function().
// Normally would have to use utils::some_module::another_module::useful_function;
// Now can just use utils::useful_function;
pub use self::some_module::another_module::useful_function;

// This is also called a re-exporting.
// It also allows for users of the module to easily find certain items.
// It's also listed in the generated docs for this module.

pub mod some_module {
    pub mod another_module {
        pub fn useful_function() {
            println!("Aren't I so useful?")
        }
    } 
}