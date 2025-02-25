/* Like tuples, structs contain related data.
Unlike tuples, structs have defined 'fields' that are unordered. */
struct User {
    active: bool,
    username: String,
    email: String,
    signed_in_count: u64,
}

/* Tuple structs are similar to tuples, but with a struct name.
Unlike regular structs, don't have names for fields */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-like structs have no fields */
struct Empty;

fn main() {
    // Create an instance of User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        signed_in_count: 1
    };

    // Use . notation to access fields in struct
    // If the instance is mutable, we can change the value of a field
    //      Note that the entire instance must be mut, Rust can't mark specific fields are immutable/mutable
    user1.email = String::from("some1@example.com");
    println!("[User 1] {} {} {} {}", user1.active, user1.username, user1.email, user1.signed_in_count);
    println!();

    //$ The struct update syntax concisely allows a struct to take the values of another struct, w/ some changes
    {
        // user2 has the same username, active, and sign_in_count as user1 - but has a different email
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1 // the (..) specifies the remaining fields, not explicitly set, should have the same values as the fields in the instance
            // ..user1 must come last to specify this
        };
        println!("[User 2] {} {} {} {}", user2.active, user2.username, user2.email, user2.signed_in_count);
        
        // The assignment (=) moves and copies data
        // Ownership over user1.username value gets moved to user2.username, since it is a String
        //      println!("{}", user1.username); //! borrow of moved value
        // active & signed_in_count are copied, so user1 still has valid data for those fields
        println!("[User 1] {} INVALID_USERNAME {} {}", user1.active, user1.email, user1.signed_in_count);
        println!()
    }

    // Tuple structs
    {
        // note that black and origin are different types, despite having fields of the same type
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("Black: {}, {}, {}", black.0, black.1, black.2);
        println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
        println!()
    }

    // Unit-like structs
    {
        let _empty = Empty;
    }

    /* You may notice that the User struct used String rather than &str.
    It is much more convenient when structs own data, but it is possible to store references.
    However, requires the use of lifetimes to do so, to be shown in a later chapter.
    */

    //$ The Rust borrow checker will track permissions at both the struct and field level
    {
        struct Point { x: i32, y: i32 }

        let mut p = Point { x: 0, y: 0 };
        // p: RWO
        // p.x: RWO
        // p.y: RWO

        let x = &mut p.x;
        // p: -
        // p.x: -
        // p.y: RWO

        *x += 1;
        // p: RWO
        // p.x: RWO
        // p.y: RWO

        println!("{}, {}", p.x, p.y)

        // Both p and p.x temporarily lose their permissions, but not p.y
    }
}

fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //$ Field init shorthand - allows var w/ same name as field to be assigned to field
        email, // Same w/ email field, don't have to write email: email
        signed_in_count: 1,
    }
}