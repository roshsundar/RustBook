/* Summary
Memory leak: accidently create memory that is never cleaned up.
Rust's features make it very difficult, but not impossible to have memory leaks.

Rc<T> and RefCell<T> create memory leaks when items refer to each other in a cycle.
The reference count of each item in the cycle will never reach 0, and thus will
never be dropped.
*/

use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};
use memory_leaks::List::{Cons, Nil};
use memory_leaks::Node;

fn main() {
    // Example of a reference cycle
    {
        // a = 5 -> Nil
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail().unwrap());
        println!();

        // b = 10 -> a, effectively 10 -> 5 -> Nil
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b inital rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail().unwrap());
        println!();

        // a = 5 -> Nil. Replace Nil with pointer to b (= 10 -> a)
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        // a = 5 -> b, effectively 5 -> 10 -> a. a now points to itself

        // Both a and b have elements that point to each other
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        println!();

        // Since both Rc's (a, b) point to each other, the memory will never be dropped.
    }

    println!("-----");
    println!();

    /*
    Calling Rc::clone() on an an Rc<T> increases its strong_count, and its dropped when the count is 0.

    You can also create a weak reference to the value in Rc<T> by calling Rc::downgrade() on it.
    Unlike strong references, weak references don't express an ownership relationship,
    and their count doesn't affect when an Rc<T> is dropped. They don't cause reference cycles.

    Rc::downgrade returns a smart pointer of Weak<T>. It increases the weak_count by 1, which
    keeps track of how many Weak<T> references exist.

    Since Weak<T> could reference a value that might have been dropped, you must make sure the value exists.
    Do this by calling upgrade() on a Weak<T>, which returns an Option<Rc<t>>.
    You'll get a Some if the Rc<T> isn't dropped yet and a None if the Rc<T> was dropped.
    */

    // Use of Weak<T> to avoid reference cycles
    {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]), // leaf has no children
            parent: RefCell::new(Weak::new()), // No parent yet
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf count strong = {}, weak = {}", // leaf has a strong reference to itself
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!();

        {
            let branch = Rc::new(Node {
                value: 5,
                children: RefCell::new(vec![Rc::clone(&leaf)]), // branch is the parent of leaf
                parent: RefCell::new(Weak::new()), // branch has no parent
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // assign leaf's parent to be branch

            println!(
                "branch strong = {}, weak = {}", // branch has a strong reference to itself and a weak reference from leaf
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade().unwrap());
            println!(
                "leaf strong = {}, weak = {}", // leaf has a strong reference to itself and a strong reference from branch
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

            println!();
        } // branch is dropped here. Its strong_count goes to 0 and the Node is dropped

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}", // leaf has a strong reference to itself
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
