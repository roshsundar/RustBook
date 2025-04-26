use std::rc::Rc;

fn main() {}

/* //1. Problem 1: returning a reference to the stack
The problem is the lifetime of the string. The reference will outlive the string.

fn _return_a_string() -> &String {
    let s = String::from("Hello World");
    &s
}
*/
//$ Solution 1.1: move ownership of string outside of function
fn _return_a_string_1() -> String {
    let s = String::from("Hello World");
    s
}
//$ Solution 1.2: return a string literal
fn _return_a_string_2() -> &'static str {
    // the 'static makes the string live forever
    "Hello World"
}
/* //$ Solution 1.3: defer borrow-checking to runtine with garbage collecting
A reference-counted pointer (Rc) can track references to data, and deallocate
the data when all Rc's to it have been dropped (at runtime).

Learn more about this in a later chapter.
*/
fn _return_a_string_3() -> Rc<String> {
    let s = Rc::new(String::from("Hello World"));
    Rc::clone(&s) // clones a pointer to s (not "Hello World")
}
//$ Solution 1.4: have the caller provide a slot to put the string
fn _return_a_string_4(output: &mut String) {
    // The mutable reference is the slot, makes the caller responsible for the memory
    output.replace_range(.., "Hello World");
}

/* //2. Problem 2: not enough permissions
The problem here is that name.push requires write permissions, but name is an immutable reference.

fn _stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
*/
/* //$ Solution 2.1: make name a mutable reference
! This works, but is a bad solution.
! Functions shouldn't mutate inputs if the caller wouldn't expect it.
A person calling this function would probably not expect the vec to be
modified by it.
*/
fn _stringify_name_with_title_1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
/* //$ Solution 2.2: take ownership of name
! This works, but is a bad solution.
! Very rare for rust functions to take ownership of heap data structures (i.e. Vec, String)
The input name also drops after this function, no longer usable in the caller.
*/
fn _stringify_name_with_title_2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
/* //$ Solution 2.3: clone name
Can mutate a local copy of the vec, preserving the original name
*/
fn _stringify_name_with_title_3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
/* //$ Solution 2.4: add the suffix later
The .join() already copies name into full, so don't have to clone
*/
fn _stringify_name_with_title_4(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

/* //3. Problem 3: aliasing and mutating a data structure
Using a reference to heap data that is dealloc'd by another reference is unsafe.
The function velow gets a references to the largest string in a vec, and then
attempts to use it while mutating the vec.

fn _add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String =
      dst.iter().max_by_key(|s| s.len()).unwrap(); // largest makes an immutable reference to data in *dst, so *dst will downgrade to immutable
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone()); // *dst is read-only, push() needs W permission
        }
    }
}
*/
/* //$ Solution 3.1: clone largest
not performant since the string must be copied into largest
*/
fn _add_big_strings1(dst: &mut Vec<String>, src: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
/* //$ Solution 3.2: perform length comparisons first, mutate dst afterwards
not performant since to_add allocates a vector
*/
fn _add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // like explained in the problem
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect(); // however, largest is dropped here, restoring *dst as mutable
    dst.extend(to_add);
}
/* //$ Solution 3.3: copy the length of largest
we don't need the contents
*/
fn _add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
/* The general idea of these solutions is to shorten the lifetime of references on dst
not overlap with mutation of dst */

/* //4. Problem 4: copying vs moving out of a collection for String
Copying a number out of a vec is straightforward

let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = &v[0];
let n: i32 = *n_ref; // value 0 is copied into n

! Can't do this with strings

let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref; //! this line errors, cannot dereference a pointer to a string

The issue is that v owns "Hello world". Dereferencing s_ref tries to take ownership,
but references can't transfer ownership.
$   - This is done to prevent double-free
    - s would drop and dealloc "Hello World", and then v does it again
    - undefined behavior

v[0] is a String, so even trying the following wouldn't be allowed
let v: Vec<String> = vec![String::from("Hello world")];
let s: String = v[0]; //! can't move "Hello world" ownership to s, only v can own it

Remember, assigning a String to another String var will attempt to move ownership.


$ In general - if a value owns heap data, it can't be copied w/o a move (ownership transfer).
i.e:
    - i32 doesn't own heap data, so it can be copied w/ no move
    - String does own heap data, can't be copied w/o move
    - &String doesn't own heap data, so it can be copied w/ no move
! one exception is a &mut references, since there can only be one mutable reference to some data at a time.

? So, if there is a vec of non-copy types, then how can we access its elements ...
*/
/* //$ Solution 4.1: just use immutable references
Avoid taking ownership.
*/
fn _get_string_from_vec1() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
}
/* //$ Solution 4.2: clone the string
Effectively allocates a new string and copies the data over, leaving the ownership
of the original string with v
*/
fn _get_string_from_vec2() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s = v[0].clone();
    s.push('!');
    println!("{s}");
}
/* //$ Solution 4.3: move the string out of v
*/
fn _get_string_from_vec3() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

/* //5. Problem 5: mutating different tuple fields
! Rust can reject safe programs due to borrow checker limitations

Take the example below of mutating a tuple.

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn _tuple_mutate() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    ); // Here name, name.0, name.1 all have RWO permissions

    let first = get_first(&name); // Here name, name.0, name.1 all lose have R permissions (lose WO)

    name.1.push_str(", Esq."); //! errors, name.1 can't be mutated as it has only R
    println!("{first} {}", name.1);
}
*/
/* //$ Solution 5.1: move the name.0 reference
For some reason, moving the name.0 reference keeps name.1 permissions
*/
fn _tuple_mutate1() {
    let mut name = (String::from("Ferris"), String::from("Rustacean")); // Here name, name.0, name.1 all have RWO permissions

    let first = &name.0;
    // name: R, name.0: R, name.1: RW

    name.1.push_str(", Esq."); // name.1 has W permissions, so this is fine
    println!("{first} {}", name.1);
}
/* The problem is that Rust looks only at get_first() function signiture,
which sees that some string gets referenced, and doesn't know which one.
It errs on caution and removes WO permissions on both name.0 and name.1 */


/* //6. Problem 6: mutating different array (or vec) elements
Take the example below.

let mut a = [0, 1, 2, 3]; // a: RWO

let x = &mut a[1];  // a: -, x: RO, *x: RW 

let y = &a[2]; //! errors, as a mutable reference has been made in the prior line
//      ^ Operation requires a: R, but doesn't have R

*x += *y;

! Same issue occurs if a was vec type instead of array

Technically though, the program is safe, since different elements are being referenced.
*/
/* //$ Solution 6.1: use split_at_mut
*/
fn _array_element_mut1() {
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
}
/* //$ Solution 6.2: directly use unsafe block
Unsafe code can work around the borrow checker, as it does not check for pointer safety.
In fact split_at_mut uses unsafe code under the hood.
More on unsafe in future chapters.
*/
fn _array_element_mut2() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing! 
}