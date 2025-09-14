/* Summary:
This is a review of some key ownership concepts in Rust.
*/

fn main() {
    /* Ownership at runtime:
    - Rust allocates variables in stack frames when a function is called, and dealloc'd when
      the call ends.
    - Local vars can hold data or pointers to data.
    - Pointers can be created through boxes (pointers owning heap data)
      or references (non-owning pointers)
    */
    {
        let mut a_num = 0;
        inner(&mut a_num);
        println!("a_num: {a_num}");
        println!()
    }

    // Slices are a special kind of reference to contiguous data in memory
    {
        let s = String::from("abcdefj");
        let s_slice = &s[2..5];
        println!("s_slice: {s_slice}");
        println!()
    }
    
    //$ Rust track R (read), W (write), O (own) permissions on each var
    // Rust operations require appropriate permissions

    {
      /*
      let n = 0; // n: RO

      n += 1; //! error, requires W
      */
    }

    // A var's permissions can change if it is moved or borrowed.
    // A move of a var w/ non-copy type (i.e. Box, String) requires RO permissions,
    // and the move deletes all original permissions on the var.
    {
      let s = String::from("Hello world");
      // s: RO

      consume_a_string(s); // operation requires RO
      // s: -

      // println!("{s}"); //! error, s requires R
    }

    // An immutable borrow disables borrowed data from being mutated or moved.

    // Printing an immutable reference is ok
    {
      let s = String::from("Hello");
      let s_ref = &s;
      println!("{s}");
      println!("{s_ref}");
    }

    // Mutating an immutable reference is not ok
    {
      let mut _s = String::from("Hello");
      let s_ref = &_s;
      // *s_ref: R 

    //s_ref.push_str(" world"); //! error, s_ref rquires W
      println!("{s_ref}");
    }

    // Mutating the immutably borrowed data is not ok
    {
      let mut _s = String::from("Hello");
      // _s: RWO

      let s_ref = &_s;
      // _s: R

    //s.push_str(" world"); //! error, s requires W
      println!("{s_ref}");
    }

    // Moving data out of the reference is not ok
    {
      let s = String::from("Hello");
      let s_ref = &s;
      // *s_ref: R

    //let s2 = *s_ref; //! error, *s_ref requirs O
      // Equivalent to let s2 = s; which tries to transfer ownership of "Hello" 

      println!("{s_ref}");
    }

    // A mutable borrow disables the borrowed data from being read, written or moved

    // Mutating a mutable reference is ok
    {
      let mut s = String::from("Hello");
      // s: RWO

      let s_ref = &mut s;
      // s: -
      // s_ref: RO, *s_ref: RW

      s_ref.push_str("Hello");
      // s: RWO
      // s_ref: -, *s_ref: -

      println!("{s}");
      // s: -
    }

    // Accessing the mutably borrowed data is not ok
    {
      let mut s = String::from("Hello");

      let s_ref = &mut s;
      // s: -

    //println!("{s}"); //! error, s needs R
      s_ref.push_str(" world");
    }

    // Rust permissions are designed to prevent undefined behavior

    // Use-after-free: freed memory is read or written
    {
      /*
      let mut v = vec![1, 2, 3];
      let n = &v[0];
      v.push(4); //! error, v needs W
      println!("{n}"); // n points to invalid memory, as v.push() has reallocated the vec somewhere else
      */
    }

    // Double-free: memory is freed twice
    {
      /*
      let v = vec![1, 2, 3];
      let v_ref = &v;
      let v2 = *v_ref; //! error, *v_ref needs O
      drop(v2); // dropping v2 deallocs v
      drop(v);  // dealloc v again, double free occurs here
      */
    }
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let _a_stack_ref = &another_num;

    let a_box = Box::new(2);  
    let _a_box_stack_ref = &a_box;
    let _a_box_heap_ref = &*a_box; // points directly to 2 on the heap

    *x += 5; // increment a_num by 5
    /* Think of it like this:
    substitute x for its value: *(&mut a_num) += 5
    dereference cancels &mut: a_num += 5
    */
}

fn consume_a_string(_s: String) {
  // om nom nom
}