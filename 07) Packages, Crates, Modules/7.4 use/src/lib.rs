/* Summary:
The *use* keyword allows an item's path to be brought into scope as a shortcut.
Once it is in scope, the shortcut can be used instead of the full path everytime you need it.
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Brings hosting module into scope, as if it's defined in the crate root
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // The *use* allows hosting to be used directly
    hosting::add_to_waitlist();
}

// Note that *use* only creates the shortcut in the scope where it's declared
mod customer {
    // The hosting use is not in scope for this module.
    // Would need to use crate::front_of_house::hosting; here too
    pub fn eat_at_restaurant() {
        //hosting::add_to_waitlist(); //! err: Rust does not recognize what hosting is
    }
}

/*
For functions, the idiomatic way to bring them into scope is to bring their parent module,
rather than the function itself. i.e:
    use crate::front_of_house::hosting;
instead of the full path
    use crate::front_of_house::hosting::add_to_waitlist;

For structs, enums, etc it's idiomatic to specify the full path.
    - i.e.
        use std::collections::HashMap;
        let mut map = HashMap::new();
The exception is when 2 items from different modules have the same name.
    - In that case, bring the parent modules of each item into scope instead
    - i.e.
        use std::fmt;
        use std::io;

        fmt::Result
        io::Result<()>
*/

/*
Another way to solve the problem of 2 items of the same name is to use the *as* keyword to rename one.
i.e.

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}

fn function2() -> IoResult<()> {}
*/

/*
Can use *pub* infront of *use* in order to make the shortcut to the item public.
This is called *re-exporting*, since we're bringing an item into scope, but also making it available for others to do the same.

i.e:
*/
mod customer_seating {
    pub mod tables {
        pub fn check_availability() {}
    }
}
// This makes tables available to external code without customer_seating needing to be pub.
pub use crate::customer_seating::tables;

/*
Nested paths allow use statements to be condensed.
i.e.
    use std::cmp::Ordering;
    use std::io;
is equivalent to
    use std::{cmp::Ordering, io};

i.e
    use std::io;
    use std::io::Write;
is equivalent to
    use std::io::{self, Write};
*/

/*
The glob operator (*) will bring all public items in a path into scope
i.e. use std::collections::*;
will bring all public items in std::collections into scope
*/