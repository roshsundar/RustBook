/* Summary

    - Each heap data allocation must be owned by exactly one variable
    - Rust deallocs heap data once its owner goes out of scope
    - Ownership can be transferred by moves
        - happens on assignments and function calls
    - Heap data an only be accessed through its current owner, not a previous owner
*/

fn main() {
    {
        // Vars will normally live on stack
        // Good for singular values, but not for collections of data
        let a = [0; 5];
        let b: [i32; 5] = a; // b is copying the whole array from a, inefficient!
        println!("{:?}", b);

        // You can easily see how this is a problem for 1,000,000 elements!
        // Allocating to the heap would allow for access to data w/out copying
    }

    {
        // Box allows data to be stored persistanly in the heap
        // It is like a pointer (like c), but smarter
        // The var it is assigned to *owns* the box
        let a = Box::new(89);

        // b now has same pointer as a
        let b = a; /* when the Box is assigned to a new var, ownership *moves* to the var.
                   this means b is now responsible for the box. */
        println!("{b}");

        // Box is deallocated/freed automatically when the owning variable's scope ends
        // Manual freeing in rust is not allowed
        // b's scope ends here, so its Box is freed
    }

    {
        // Functions that don't return a box will deallocate it

        let my_box = Box::new(10); // my_box owns 10
        consume_box(my_box);
        // my_box points to deallocated memory
    }

    {
        // Functions that return a box do not deallocate it

        let my_box = Box::new(10);
        let the_box = keep_box(my_box);
        println!("Box is still accessible: {the_box}");
        /*  Note that trying the print my_box will error.
        This is b/c ownership transfers my_box -> boxed_value -> the_box.
        my_box no longer owns 10, using it is forbidden. */
    }

    {
        // Some of rust's default data structures use boxes, like String

        let first = String::from("Ferris"); // First owns "Ferris"
        let full = add_suffix(first); // After function call, full owns "Ferris Jr."
        println!("{full}");

        // first can no longer be used since ownership has passed to full
        // In general: if x moves ownership of heap data to y, then x can't be used
    }

    {
        // One way to avoid moving data is clone(), which copies data

        let first = String::from("Ferris"); // first has "Ferris"
        let first_clone = first.clone(); // first_clone has its own "Ferris"
        let full = add_suffix(first_clone); // full owns "Ferris Jr."
        println!("{full}, originally {first}");
    }

}

fn consume_box(box_arg: Box<i32>) {
    // box_arg owns 10

    println!("Box({box_arg}) is dropped");
    // 10 is deallocated here
}

fn keep_box(boxed_value: Box<i32>) -> Box<i32> {
    println!("Box contains: {}", boxed_value);
    boxed_value // Ownership is returned to the caller
}

fn add_suffix(mut name: String) -> String {
    // name owns "Ferris"

    name.push_str(" Jr."); // name now points to "Ferris Jr."
    name
}
