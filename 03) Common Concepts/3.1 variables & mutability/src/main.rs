
// consts never mutable
// can be declared globally like here (unlike mut)
// explicitly typed
// must know value at compile time
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    { // scope - vars are local to this scope
        println!("MUTABILITY");
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6; // compile error if x isn't mut
        println!("The value of x is: {x}");
    }

    {
        println!("\nCONST");
        println!("3 hrs in seconds {THREE_HOURS_IN_SECONDS}");
    }

    {
        // shadowing essentially creates new var w/ same name, but different value, type, or mut status
        println!("\nSHADOWING");
        
        /* change value */
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
        println!("The value of x is: {x}");

        /* change type */
        let spaces = "    ";
        let spaces = spaces.len();
        println!("{spaces}");

        /* change mutability */
        let i = 6;
        let mut i = i;
        println!("i before mutate {i}");
        i = 7;
        println!("i after mutate {i}");
    }
}
