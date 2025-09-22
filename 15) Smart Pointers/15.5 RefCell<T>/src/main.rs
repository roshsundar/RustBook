/* Summary:
Interior mutability allows you to mutate data even when there are immutable references to data.
Normally this is not allowed, but RefCell<T> allows for it.

RefCell enforces the borrowing rules at runtime as opposed to compile time. This allows for some correct
code, that the borrow checker would reject at compile time, to be used.
*/

use std::cell::RefCell;

fn main() {
    /*
    Rust doesn't allow for mutating an immutable value, i.e.
    let x = 5;
    let y = &mut x;
    But there are some reasons to do so. Look at src/lib.rs for an example
    */

    /*
    When creating immutable and mutable references, we use the & and &mut syntax.
    With RefCell<T>, we use the borrow() and borrow_mut() methods.

    borrow() returns the smart pointer type Ref<T>. borrow_mut() returns the smart pointer type RefMut<T>.

    RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are active at runtime.
    RefCell checks to make sure that there are either many Ref's or one RefMut at a given time.

    If these rules are violated, rather than getting a compiler error as we would with references,
    RefCell will panic! at runtime.
    */

    /*
    RefCell<T> is often used w/ Rc<T>. Recall that Rc<T> allows for multiple owners for data, but only w/ immutable access.

    If you have an Rc<T> that holds a RefCell<T>, you can have multiple owners and can mutate. 
    */
    {
        use std::rc::Rc;

        // Shared mutable data, a counter
        let counter = Rc::new(RefCell::new(0));
        println!("counter: {}", counter.borrow());

        // Clone the Rc to get 2 more owners of counter
        let c1 = Rc::clone(&counter);
        let c2 = Rc::clone(&counter);

        assert_eq!(3, Rc::strong_count(&counter)); // There are 3 owners of the counter: counter, c1, c2

        // Modify through the first owner
        *counter.borrow_mut() += 1;
        println!("counter: {}", counter.borrow());

        // Modify through the second owner
        *c1.borrow_mut() += 1;
        println!("counter: {}", c1.borrow());

        // Modify through the third counter
        *c2.borrow_mut() += 1;
        println!("counter: {}", c2.borrow());
    }
}
