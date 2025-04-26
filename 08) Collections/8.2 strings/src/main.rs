/* Summary:
String is a growable, mutable, owned, UTF-8 encoded string type.
    - The &str type is also sometimes referred to as a 'string', also must be valid UTF-8

Strings can be concatenated w/ the + operator and format! macro.

Strings are vectors of (UTF-8) bytes fundementally, which can represent several granularities of sequenced data.
As such, direct indexing into a String is not allowed, but creating string slices is. 
Be careful that the range of bytes provided spans valid characters.

Strings can be iterated over as chars or bytes.
*/

fn main() {
    // Create a String
    {
        let mut s = String::new(); // create an empty String
        println!("{s}");

        // Load a String with data
        let data = "initial contents";
        s = data.to_string();
        println!("{s}");
        s = "initial contents".to_string();
        println!("{s}");
        s = String::from("initial contents");
        println!("{s}");

        // Strings can store any type of UTF-8 data
        let _ = String::from("Hello");
        let _ = String::from("السلام عليكم");
        let _ = String::from("안녕하세요");

        println!()
    }

    // Append a String
    {
        // Use push_str() to append a string slice to a String
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // a &str is used so the var passed in can be used after
        println!("foo + {s2} = {s1}");

        // push() appends a single char to a String
        let mut s = String::from("lo");
        s.push('l');
        println!("{s}");

        println!()
    }

    // Concatenate Strings
    {
        // The + operator can add 2 Strings
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note: s1 has been moved here and can no longer be used, but s2 can b/c it's a reference
        /* 
        The + operator calls an add(self, &str) function
            - 2 String types can't be added together, only String and &str
            - self takes ownership of the String provided to it, so it can't be used after
            - If &String is provided instead of &str, then it is auto-converted into &str.

        let s3 = s1 + &s2; does the following:
            1. add takes ownership of s1,
            2. it appends a copy of the contents of s2 to s1,
            3. and then it returns back ownership of s1.
        */
        println!("{s3}");

        // For concatenating multiple Strings, + is unwieldy
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{s}");

        // The format! macro can be used for complicated concatenation
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{s1}-{s2}-{s3}"); // format! only takes in references, so all Strings can be used after
        println!("{s}");

        println!()
    }

    /* Strings can't be indexed, Rust doesn't allow it
    let s1 = String::from("hello");
    let h = s1[0]; //! err
    */

    /*
    A String is a wrapper over a Vec<u8>, a list of bytes.

    The String "hola" is len=4, so 4 bytes long in UTF-8. However, the String "Здравствуйте" is len=12, but is 24 bytes long.
    Each character in that String is 2 bytes long. Therefore, indexing into 0 will only get 1 out of 2 bytes of the first character,
    which would be invalid.
    */
    /*
    There are 3 ways to look at Strings: as bytes, scalar values, and grapheme clusters (closest to letters).

    Take the Hindi word “नमस्ते”.
    As a vector of bytes, it looks like this:
        - [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 
            224, 164, 164, 224, 165, 135]
        - This is how it is stored in memory
    As unicode scalar values (char type):
        - ['न', 'म', 'स', '्', 'त', 'े']
        - There are 6 chars here, 4th & 6th are accents
    As grapheme clusters:
        - ["न", "म", "स्", "ते"]
        - These are the letters that make up the word
    */

    // Slicing Strings
    {
        // Indexing a String is often not advised, since the return could be
        // a byte vale, character, grapheme cluster, or string slice.

        // Index w/ a range to create a string slice
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{s}");

        /*
        If you slice only part of a char's bytes, i.e. &hello[0..1],
        it would panic at runtime as if a vector was indexed out of bounds.
        !   byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`

        $ Use caution when creating String slices w/ ranges, as it can crash the program.
        */

        println!()
    }

    // Iterating over Strings
    {
        // chars() returns an iteration of the characters in an &str.
        for c in "Зд".chars() {
            println!("{c}");
        }

        // bytes() does a similar thing and returns the raw byte.
        //$ Remember that valid Unicode scalar values may be made up of more than one byte.
        for b in "Зд".bytes() {
            println!("{b}");
        }

        println!()
    }
}
