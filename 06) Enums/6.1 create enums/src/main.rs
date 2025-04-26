/* Summary:
Enums allow a value to be one of several (mutually exclusive) types.
    - These types are called *variants*

These variants can have their own associated data.
This allows functions to take in one enum, and do something different for each type.

Options are a special enum to handle valid/invalid situations.
*/

enum Message {
    _Quit, // no data
    _Move {x:i32, y: i32}, // named fields, like a struct
    Write(String), // a String
    _ChangeColor(i32, i32, i32), // 3 i32 values
}

// Just like structs, enums can have methods defined on them
impl Message {
    fn print_write_message(&self) {
        if let Message::Write(message) = self {
            println!("Write: {message}");
        }
    }
}

fn main() {
    {
        let m = Message::Write(String::from("hello"));
        m.print_write_message();
    }

    {
        /*
        Rust provides a special enum called Option, to deal with a scenario where a value could be something or nothing.
        Option has a Some and None variant. Option can be any type, Option<T>
        */

        // Rust can infer the type for Some
        let _some_num = Some(5);
        let _some_char = Some('e');
        // Need to specify type (i32 in this case) for None
        let _absent: Option<i32> = None;

        // With a Some value, we know a value is present and is contained in Some.
        // With a None, we don't have a 'valid' value.

        /*
        Rust will not allow an Option<T> to be used as if it were T

        let x: i32 = 5;
        let y: Option<i32> = Some(5);
        let sum = x + y; //! err: Option<i32> can't be added to i32

        Need to convert the Option<i32> to i32 to use it in regular i32 operations.
        */

        /*
        To use an Option<T>, you must handle each variant - Some(T) and None.
        If the Option<T> is Some(T), you run some code and can use the T value.
        If the Option<T> is None, you run some other code and there is no T value.
        */
    }
}
