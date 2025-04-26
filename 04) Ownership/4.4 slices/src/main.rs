/* Summary:
Slices are a special kind of reference that refer to sub-ranges of a sequence, like a string or a vector.
At runtime, a slice is represented as a “fat pointer” which contains a pointer to the beginning of the range and its length.
*/

fn main() {
    /*
    To understand why slices are useful, let's start w/ a small problem.
    
    Problem: take a string of words and find the first word

    We could return the index that marks the end of the word.
    The problem is that it returns a usize on its own, and is not 'connected' to the string.
    Because it is separate from the string, there is no gauarantee that it's valid later
    */
    {
        let mut s = String::from("Hello world");
        let _word = first_word_index(&s);
        s.clear(); // s = ""

        // As mentioned above, the _word index doesn't correspond to the string anymore.
        // We have to manually keep in mind to sync the index and string.
        // This is prone to logic bugs.
    }

    // Rust provides string slices to reference a part of a string
    {
        let s = String::from("hello world");
        let _hello = &s[0..5];
        let _world = &s[6..11];

        // The range is [starting_index..ending_index+1]
    }

    // Slices are references, so they also change permissions on referenced data
    {
        let mut s = String::from("hello");
        // s: RWO
        let hello = &s[0..5];
        // s: R
        // hello: RO
        // *hello: R

        println!("{hello}");
        // s: RWO
        // hello: -
        // *hello: -

        s.push_str(" world");
        // s: -

        // like any other immutable reference
    }

    // Range syntax
    {
        // If index should start at 0, drop the value before ..
        // i.e. 0..2 and ..2 are equivalent
        let s = String::from("hello");
        let s1 = &s[0..2];
        let s2 = &s[..2];
        assert!(s1 == s2);

        // If the slice includes the last byte of the String, you can drop the trailing number
        let s = String::from("hello");
        let len = s.len();
        
        let s1 = &s[3..len];
        let s2 = &s[3..];
        assert!(s1 == s2);
    }

    // String literals are slices
    {
        let _s = "Hello world!"; // points to a specific address in the binary
    }
    
    // Getting first word through a slice
    {

        let s = String::from("hello world");
        let word = first_word_slice(&s);

        // Since slices change permissions, Rust can catch the empty String problem.
        // i.e. if we did s.clear(), it would error since s only has R permissons
        // after the &s

        println!("the first word is {word}");
    }

    // The first word function taking in an &str allows it to work with &String and &str values
    {
        // the fn works on slices of Strings, partial or whole 
        let my_string = String::from("hello world");
        let _word = first_word_improved(&my_string[0..6]);

        // it also works on references to Strings, equivalent to whole slices
        let _word = first_word_improved(&my_string);

        let my_string_literal = "hello world";

        // the fn works on slices of string literals, partial or whole
        let _word = first_word_improved(&my_string_literal[0..6]);
        let _word = first_word_improved(&my_string_literal[..]);

        // Because string literals are string slices already,
        // this works too, without the slice syntax!
        let _word = first_word_improved(my_string_literal);
    }

    // Other slices
    {
        // There is a more generic slice type for types other than Strings

        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2,3]);
    }
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // return a slice of the String up the the space
        }
    }

    &s[..] // return the whole String as a slice
}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // return a slice of the String up the the space
        }
    }

    &s[..] // return the whole String as a slice
}