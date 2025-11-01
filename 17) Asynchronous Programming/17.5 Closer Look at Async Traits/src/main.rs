/* Summary:
Let's examine the Future, Pin, Unpin, Stream, and StreamExt traits in more detail.
*/

use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    // The Future trait.
    {
        // This is how Rust defines the Future trait:
        pub trait Future {
            type Output; // Output is the type the future resolves to.

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        }

        // The poll() method returns the Poll type.
        // When a future is Pending there is work to do. Ready indicates the future is finished and T is available.
        enum Poll<T> {
            Ready(T),
            Pending,
        }

        // Whenever you see .await in async code, Rust compiles it under the hood to code that calls poll().
        // If the Poll is Pending, then it can pause the current future and work on other ones.
    }

    // The Pin & Unpin trait.
    {
        // As seen from the definition of the Future trait, poll() only works on futures wrapped in a Pin.

        //$ Pin is a wrapper for pointer-like types such as &, &mut, Box, and Rc. It enforces constraints on pointer usage.

        // When we pin a value by wrapping a pointer to it in Pin, it can no longer be moved in memory.
        // If you have a Pin<Box<SomeType>>, you pin the SomeType value, keeping it in the same place in memory.
        
        //$ Only need to pin values that have internal references, such as futures!
        // Primitive types such as numbers and booleans need not be pinned, as they've no internal refernces.
        // If you had a Pin<Vec<String>>, you'd have to use the Pin methods to interact w/ it,
        // even though a Vec,String> is safe to move if there are no other references to it.
        
        // Unpin is a marker trait w/ no functionality of its own.
        //$ Unpin informs the compiler that a type doesn't need to uphold any guarantees about whether it can be safely moved.
        // It's implemented automatically for types it can verify as safe.
        // For types where it's no implemented, it's notated as impl !Unpin for SomeType.

        // Unpin is the 'normal' case and !Unpin is the special case.
        // Whether a type implements Unpin or !Unpin only matters when using a pinned pointer to it, i.e. Pin<&mut SomeType>.

        /* Example:
        Think about a "hello" String. We can wrap it in a Pin - Pin<String>.
        
        String automatically impl's Unpin, so you can do things that would be illegal if String were !Unpin instead.

        Replacing one String w/ another in the same memory location is allowed b/c String is Unpin.
        i.e. replacing the "hello" in Pin<String> w/ "goodbye". 
        */
    }

    // The Stream and StreamExt traits.
    {
        // Remember that Streams are a sequence of items that become ready over time.
        // As such, you can think of it as an iterator over futures.

        trait Stream {
            type Item; // Associated type of the items produced by the stream.

            // poll_next() is kinda like a combo of Future:poll() (in that it polls a future to completion),
            // and Iterator::next() (produces a sequence of items).
            fn poll_next(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>
            ) -> Poll<Option<Self::Item>>; // The outer type of the return type is Poll, as it has to be checked for readiness.
                                           // The inner type is Option, as it needs to say if there are more items in the sequence.
        }

        // The StreamExt trait makes it easier to work with streams, letting us use .await and next(), among many other useful methods.
    }
}
