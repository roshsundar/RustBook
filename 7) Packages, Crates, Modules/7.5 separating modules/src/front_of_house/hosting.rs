// The hosting module has this function
pub fn add_to_waitlist() {}

/*
If hosting.rs was located in src, the compiler would expect hosting module to be under the crate root,
rather than the child of the front_of_house module.

Note that file path matches the module hierarchy:
src/front_of_house/hosting.rs
crate::front_of_house::hosting
*/