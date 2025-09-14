/* Summary:
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
Vectors can only store values of the same type.
*/

fn main() {
    // Create a vector
    {
        // An empty vector must specify the type
        let _: Vec<i32> = Vec::new();
        
        // vec![] macro creates vector w/ initial elements. The type doesn't need to be specified.
        let _ = vec![1, 2, 3];
    }

    // Update a vector
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
    }

    // Reading vectors
    {
        // 2 ways to reference value in a vector

        let v = vec![1, 2, 3, 4, 5];

        // One way is to make a reference directly to the element
        let third = &v[2];
        println!("The third element is {third}");

        // Another way is to use get() which returns an Option w/ reference to the element
        // This way is useful if you aren't sure if the vector is long enough to contain the index
        let third = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        println!()
    }

    // Rust enforces ownership and borrowing rules on vectors
    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0]; // immutable reference to data in v
    
        //v.push(6); //! err: attempting to mutate v that is borrowed as immutable
    
        println!("The first element is: {first}");

        println!()
    }

    // Iterating over a vector
    {
        // Immutably borrow a vector and iterate
        let v = vec![100, 32, 57];
        for n_ref in &v {
            let n_plus_one = *n_ref + 1;
            println!("{n_plus_one}");
        }

        // Mutably borrow a vector and iterate
        let mut v = vec![100, 32, 57];
        for n_ref in &mut v {
            *n_ref += 1;
        }

        println!()
    }

    // Iterators
    {
        // Iterator is a pointer that can increment through each element of a vector.
        // It returns an Option, with a Some that can be unrwapped into a reference to the element,
        // or None if it has gone beyond the end of the vector.
        let v = vec![1, 2];
        let mut iter = v.iter();
        let n1 = iter.next().unwrap();
        let n2 = iter.next().unwrap();
        let end = iter.next();
        println!("{n1} {n2} {end:?}");

        // iter makes an immutable referece to an element in the vector,
        // so can't mutate the vector while the iterator is in use. 
        
        // Can also iterate over a vector through a Range
        let v = vec![1, 2];
        let mut iter = 0 .. v.len();
        let i1 = iter.next().unwrap();
        let n1 = v[i1];
        println!("{n1} at index {i1}");

        println!()
    }

    // Using Enums to store multiple types
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        // The vec can effectively store three different types through the SpreadsheetCell enum
        // A match can be used on an element to extract the int, float, or text
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("hello")),
            SpreadsheetCell::Float(10.12),
        ];
        
        println!("{row:?}");
    }
}
