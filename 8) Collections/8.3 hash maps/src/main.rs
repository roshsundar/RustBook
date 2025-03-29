/* Summary:
The type HashMap<K, V> stores a mapping of keys of type K to values of type V.
Each key must be unique, values need not be.

Hash maps store data on the heap.
*/

use std::collections::HashMap;

fn main() {
    // Creating HashMaps
    {
        let mut scores = HashMap::new();
        
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    // Accessing values in a HashMap
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");

        // get() returns an Option<&i32>. copied() converts it to Option<i32>.
        // unwrap_or gets the score, and defaults to 0 if there isn't a key for "Blue".
        let _score = scores.get(&team_name).copied().unwrap_or(0);

        // HashMaps can be iterated like vectors
        // The order of key-value pairs is arbitrary
        for (key, value) in &scores {

            println!("{key}: {value}");
        }

        println!()
    }
    
    // HashMap ownership
    {
        // For types with Copy trait, like i32, the values are copied into the hash map.
        // For owned values, like String, they are moved into the hash map - which becomes the owner.

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value); // move of field_name and field_value into map

        // field_name and field_value are invalid at this point

        // Since hash maps are heap-owned, they also transfer ownership
        let mut _new_map = map;

        // map is invalid at this point
    }

    // Updating a HashMap
    {
        // insert() with the same key will replace the previous value
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{scores:?}");

        println!();

        // You might want to check if a key w/ value exists in the hash map
        //      - keep it the same if it does
        //      - insert it if it doesn't

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // entry() returns an enum Entry that represents a value that may / may not exist.
        // or_insert() returns mut ref to value for the Entry key if it exists. 
        //  If not, it inserts the param as the new value for the key, and returns a mut ref to the value. 

        scores.entry(String::from("Yellow")).or_insert(50); // "Yellow" doesn't exist in scores, so add it with a key of 50
        scores.entry(String::from("Blue")).or_insert(50); // scores contains "Blue" already, so it won't be modified

        println!("{scores:?}");

        println!();

        // Hash maps are often used to look up a key's value and update it based on the old value.
        // The below will count the existance of words in a text

        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // Check if the word exists in map, and insert it w/ a count value of 0 if it doesn't.
            // Return a mut ref to the count of the word, increment by 1.
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{map:?}");
    }

    {
        let mut h: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in "hello!".chars().enumerate() {
          h.entry(c).or_insert(Vec::new()).push(i);
        }
        let mut sum = 0;
        for i in h.get(&'l').unwrap() {
          sum += *i;
        }
        println!("{h:?} {}", sum);
    }
}
