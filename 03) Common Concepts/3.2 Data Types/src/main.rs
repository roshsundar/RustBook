use std::io;

fn main() {
    /*  Rust is statically typed. Compiler can guess type in many instances,
        but sometimes must be explicitly declared.

        The assignment below will error if type isn't specified, as parse()
        needs to know what kind of integer to convert "42" to.
    */
    let _:u32 = "42".parse().expect("Not a number!"); // the _ means we don't care abt result, done so I don't have to print everything

    /* Scalar type represents single value: integers, floating-point numbers, booleans, and characters */

    {
        /*  Integer types details can be found here: https://rust-book.cs.brown.edu/ch03-02-data-types.html 
            In summary, there are 6 sizes & a signed/unsigned version of each
            Ints can be expressed in a variety of ways (i.e. hex)
        */

        /* Underscores can be used in int assignment to make expression easier to read */
        let _ = 1_000;  // 1_000 = 1000, doesn't change numerical value at all
    }

    {
        /* Two float types, f64 is default & recommended */
        let _ = 2.02; // f64
        let _:f32 = 3.3;
    }

    {
        // addition
        let _ = 5 + 10;

        // subtraction
        let _ = 95.5 - 4.3;

        // multiplication
        let _ = 4 * 30;

        // division
        let _ = 56.7 / 32.2;
        let _ = -5 / 3; // Results in -1

        // remainder
        let _ = 43 % 5;
    }
    
    {
        let _ = true;
        let _: bool = false; // with explicit type annotation
    }

    {
        let _ = 'z';
        let _: char = 'ℤ'; // with explicit type annotation
        let _ = '😻';

        // chars use '', whereas strings use ""
    }

    /* Compound types group multiple values into one type: tuples & arrays */

    {
        /*  Tuples group several values of several types into one type.
            They have a fixed length, and each position can only accept its type once declared.
        */
        let tup = (500, 6.4, 1);

        // A tuple can be destructured to get each element
        let (x, y, z) = tup;
        println!("x:{x}, y:{y}, z:{z}");

        // Tuple value can be acessed directly by . followed by index
        println!("x:{}, y:{}, z:{}", tup.0, tup.1, tup.2);

        // Empty tuple () is called 'unit', used typically to represent empty val or return type

        // Individual elements of tuple value can be modified
        let mut tup = (1, 2.2);
        tup.0 = 0;
        tup.1 += 3.0;
        let (x, y) = tup;
        println!("{x}, {y}");
    }

    {
        /* Arrays group several values of one type. They are fixed length */
        let _  = ["I", "am", "an", "array!"];

        /*  Init an array with same val for each element.
            equivalent to let a = [3, 3, 3, 3, 3];
        */
        let _ = [3; 5]; // [init val; length];

        // Array elements can be accessed by arrayName[index]
        let a = [1, 2, 3, 4, 5];
        println!("first:{}, second:{}", a[0], a[1]);
    }
    
    {
        /*  Rust will check array bounds in runtime as well as compile time.
            The code below will run fine if you enter an index within bounds,
            but will panic otherwise.
        */

        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");
    
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index];
    
        println!("The value of the element at index {index} is: {element}");
    }

    {
        /* Arrays & tuples can be combined too */
        let t = ([1; 2], [3; 4]); // t = ([1, 1], [3, 3, 3, 3])
        let (a, _) = t; // a = [1, 1]
        println!("{}", a[0] + t.1[0]); // [1, 1][0] + [3, 3, 3, 3][0] = 1 + 3 = 4
    }
}