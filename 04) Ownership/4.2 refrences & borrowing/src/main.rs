/* Summary:
References provide the ability to read and write data without consuming ownership of it.
References are created with borrows (& and &mut) and used with dereferences (*), often implicitly.

However, references can be easily misused. Rust’s borrow checker enforces a system of permissions that ensures references are used safely:
    - All variables can read, own, and (optionally) write their data.
    - Creating a reference will transfer permissions from the borrowed place to the reference.
    - Permissions are returned once the reference’s lifetime has ended.
    - Data must outlive all references that point to it.
*/

fn main() {
    {
        // move behavior can be inconvenient if you want to assign a string more than once
        // references are provided to do this kind of stuff
        // A reference(&) is a non-owning pointer

        let m1 = String::from("Hello"); // m1 owns "Hello"
        let m2 = String::from("world"); // m2 owns "world"
        greet(&m1, &m2); // pass m1 and m2 as references
        println!(" was originally {m1} {m2}"); // m1 and m2 can still be used since ownership hasn't moved

        println!();
    }

    {
        // To get the data of a pointer, dereference(*) is used

        let mut x = Box::new(1);
        let a = *x; // *x gets the heap val, so a = 1
        println!("a = {a}");
        *x += 1; // increments the heap val by 1, so x -> 2

        let r1 = &x; // r1 -> x -> 2
        let b = **r1; // two dereferences, b = 2
        println!("b = {b}");

        let r2 = &*x; // pointer(&) to the value(*) of x, so r2 -> 2
        let c = *r2; // dereference r2 -> 2, so c = 2
        println!("c = {c}");

        println!();
    }

    {
        // In many cases rust allows for smart derefrencing through the (.) operator

        let x: Box<i32> = Box::new(-1);
        let x_abs1 = i32::abs(*x); // explicit dereference
        let x_abs2 = x.abs(); // implicit dereference
        assert_eq!(x_abs1, x_abs2);

        let r: &Box<i32> = &x;
        let r_abs1 = i32::abs(**r); // explicit dereference (twice)
        let r_abs2 = r.abs(); // implicit dereference (twice)
        assert_eq!(r_abs1, r_abs2);

        let s = String::from("Hello");
        let s_len1 = str::len(&s); // explicit reference
        let s_len2 = s.len(); // implicit reference
        assert_eq!(s_len1, s_len2);
    }

    /*
    ! Rust avoids data being aliased & mutated at the same time. It leads to undefined behavior.
        aliased: accessed by multiple variables

    Boxes disallow aliasing, since there is one owner with access to the data, keeping w/ the principle.
    For references, rust enforces safety through the borrow checker.

    The borrow checker allows three kinds of permissions for a var's data:
        1. Read(R): data can be copied
        2. Write(W): data can be mutated
        3. Own(O): data can be moved/dropped
    ! These permissions are compiler only, not runtime
    By default a var has RO permissions, adding let mut also adds W.
    $ references can temporarily remove these permissions
    */

    {
        let mut v = vec![1, 2, 3];
        // v: RWO

        let num = &v[2]; // The reference requires v to have R permission
        // v: R
        // num: RO
        // *num: R
        /*  v loses WO permissions. The reason v loses WO is because num would point to invalid memory if
            v was mutated or dropped.

            num gains RO, but not W, since it isn't mutable.
            The place num is pointing to, *num, has R.
        */

        println!("Third element is {}", *num);
        // v: RWO
        // num:
        // *num:
        /*  v is no longer being borrowed, so it regains permissions. 
            num is no longer in use, so num and *num lose permissions */

        v.push(4); // The push requires v to have R and W permissions
        // v:
        /*  v is no longer in use and loses all permissions */

        println!();
    }

    {
        //$ Note that permissions exist for vars(var) and places(*var)
        // Places are anything on the left-hand side of an assignment(=)

        let x = 0;
        // x: RO

        let mut _x_ref = &x;
        // _x_ref: RWO //$ Permissions for _x_ref itself
        // *_x_ref: R //$ Permissions for place _x_ref points to
        // x: R

        /*  Note that _x_ref has W while *_x_ref doesn't. So we can change _x_ref, like _x_ref = &y. 
            But we can't mutate the data it points to, like *_x_ref += 1 */
    }

    //$ The borrow checker finds permission violations

    {
        // The & references are R only, also called 'immutable references' or 'shared refernces'
        //$ The &mut references are RW, also called 'mutable references' or 'unique references' 

        let mut v = vec![1, 2, 3];
        // v: RWO

        let num = &mut v[2];
        // v:
        // num: RO
        // *num: RW
        /*
        Note that &mut caused v to lose R permissions as well, unlike &
        This is to prevebt aliasing while mutating. Otherwise, v and *num could read the same memory,
        while *num can modify.
        */

        *num += 1;
        println!("Third element is {}", *num);
        // v: RWO
        // num:
        // *num:

        println!("Vector is now {:?}", v);
        // v:

        println!();

        /*
        In general:
        1. There can be as many immutable references (&) at the same time
        2. Only one mutable reference (&mut) can exist at a time
        3. A mutable reference disallows any immutable references
        */
    }

    {
        // Mutable references can also be temporarily “downgraded” to read-only references

        let mut v = vec![1, 2, 3];

        let num = &mut v[2];
        // *num: RW

        let num2 = & *num;
        // *num: R
        // *num2: R

        println!("{} {}", *num, *num2);

        println!();
    }

    {
        /* 
        $ Permissions are returned at the end of a references's lifetime
        lifetime = range of code spanning reference creation -> last use
        */

        let mut x = 1;

        let y = &x;
        // x: R
        // start of y lifetime

        let z = *y;
        // x: RWO
        // end of y lifetime, x permissions restored

        x += z;

        println!("{x}");
        
        println!();
    }

    {
        // A lifetime can also be discontiguous when control flow is introduced

        let mut v = vec!['a', 'b', 'c'];
        
        let y = &mut v;
        // *y: RW

        let c = & y[0];
        // *y = R
        // Downgrade mutable reference

        if c.is_ascii_lowercase() {
            let up = c.to_ascii_uppercase();
            // *y = RW
            // In this block of if, c's immutable reference is dropped here
            // The original mutable reference permissions for *y are restored

            y[0] = up;
            println!("Capitalized: {:?}", y);
        }
        else {
            println!("Already capitalized: {:?}", y);
        }
    }

    {
        /*
        ! The borrow checker enforces that data must outlive any references to it.
        In most cases, rust knows how long a reference lives.
        However, rust doesn't know lifetime of a reference when:
            - reference is input or output for a function

        i.e. creating a str in a func and trying to return a reference to it.
        rust doesn't know when the str will stop being used, so it can't dealloc it when the function ends.

        $ For these kinds of references rust uses Flow(F) permission to check safety.
        Unlike RWO, F doesn't change in the body of a function.
        A reference has F if it is allowed to be used in an expression.
        More on this in another chapter.
        */
    }
}

fn greet(g1: &String, g2: &String) {
    // g1 -> m1 -> "Hello" - g1 references m1
    // g2 -> m2 -> "world" - g2 references m2

    print!("{g1} {g2}!");

    /*  g1 and g2 are dropped here,
    but since they don't directly own "Hello" and "world",
    those strings aren't deallocated */
}