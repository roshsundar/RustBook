/* Summary:
The Drop trait lets you customize what happens right before a value of a type goes out of scope.

It is often used with smart pointers, like when Box<T> dealloc's data on the heap.

It also eliminates the need for cleanup code everywhere, just need to define it in one place.
*/

fn main() {
    struct CustomSmartPointer {
        data: String,
    }

    impl CustomSmartPointer {
        fn new(data: String) -> Self {
            println!("CustomSmartPointer created with data '{data}'");
            Self { data }
        }
    }

    impl Drop for CustomSmartPointer {
        // This function will be run when a CustomSmartPointer is dropped
        fn drop(&mut self) {
            println!("Dropping a CustomSmartPointer with data '{}'", self.data)
        }
    }

    // Example of the custom Drop logic running
    {
        let c = CustomSmartPointer::new(
            String::from("my stuff")
        );

        // The drop logic for CustomSmartPointer will be called here,
        // as this is where 'c' goes out of scope
    }

    println!();

    // Manually drop with drop()
    {
        let c = CustomSmartPointer::new(
            String::from("some data")
        );

        // Manually force the drop before the end of scope
        drop(c);

        println!("CustomSmartPointer was dropped early!");

        // Would've been dropped here
    }
}
