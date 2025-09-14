/* Summary:
The iterator pattern allows you to perform some task on a sequence of items in turn. 
Iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
Iterators have many methods that can oprate on them to make complex transformations of collections.
*/

fn main() {
    // Immutable item iterator
    {
        let v = vec![1, 2, 3];
        
        // iter() iterates with immutable references to each item
        let v_iter = v.iter();

        // The iter handles the incrementing and indexing to access each element
        for val in v_iter {
            print!("{val} ");
        }

        println!();
    }

    // Mutable item iterator
    {
        let mut v = vec![1, 2, 3];
        
        // iter_mut() iterates with mutable references to each item
        for val in v.iter_mut() {
            *val += 1;
        }
        println!("{v:#?}");
    }

    // Owned item iterator
    {
        let v = vec![1, 2, 3];
        
        // into_iter() transfers ownership of each item out of v
        for val in v.into_iter() {
            print!("{val} ");
        }
        // println!("{v:#?}") // !❌ err: v has been moved and is no longer accessible

        println!()
    }

    // Iterators can be manually incremented w/ the next() method
    {
        let v = vec![1, 2, 3];

        // The iter needs to be mut, since next() changes the internal state of the iter.
        // Each call to next() uses up the iterator.
        let mut v_iter = v.iter();

        // The next() method returns an Option. Either Some w/ reference to the item, or None if at the end of the iter
        assert_eq!(v_iter.next(), Some(&1)); 
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }

    // Consuming adaptors
    {
        // Consuming adaptors: a variety of methods are defined on iterators that call next() and consume the iterator.
        // i.e. sum()

        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        assert_eq!(total, 6);
    }

    // Iterator adaptors
    {
        // Iterator adaptors: a variety of methods are defined on iterators that modify the iterator.
        // They return a new iterator that is the modified original.
        // i.e. map()

        let v = vec![1, 2, 3];
        
        // map() method takes a closure to call on each iterated item
        let v_iter = v.iter().map(|x| x + 1);

        // collect() method consumes the iterator and outputs a specified collection
        let v_plus_1: Vec<i32> = v_iter.collect();

        assert_eq!(v_plus_1, vec![2, 3, 4]);

        // This pattern of chaining iterators and then collecting can allow for complex transformations
    }
    
    // Iterator closures can capture their enviro
    {
        let scores = vec![99, 3, 55, 20];

        let threshold = 50;

        let passing_scores: Vec<i32> = scores.into_iter()
            // filter() takes a closure, which gets an iterated item and returns a bool.
            //      If the bool is true, the item will be included in the new iterator.
            //      If the bool is false, it won't be included.
            // filter() gives a reference to each item in the iter, &i32. &score destructures it to get an i32.
            .filter(|&score| score >= threshold) 
            .collect();

        // Notice how the filter() cosure captures threshold by immutable borrow

        assert_eq!(passing_scores, vec![99, 55]);
    }

    // Complex examples
    {
        let v = vec![1, 2, 3, 4];

        let a: Vec<_> = v.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();
        assert_eq!(a[0], 4);

        let b: Vec<_> = v.iter()
            .map(|x: &i32| x * 2)
            .filter(|x: &i32| x % 2 == 0)
            .collect();
        assert_eq!(b[0], 2);
    }
}
