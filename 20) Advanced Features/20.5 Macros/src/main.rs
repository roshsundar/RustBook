/* Summary:
Macros in Rust refer to several things:
• Declarative macros, w/ macro_rules!
• 3 procedural macros:
    • Custom #[derive] macros used on structs and enums
    • Attribute-like macros define custom attributes usable on any item.
    • Function-like macros look like function calls but operate on the items specified as their argument

Macros are a way of writing code that generates other code.
    • i.e. The println! and vec! macros expand to produce more code that what was written

This may sound similar to functions but there are some differences:
    • Function signatures take in a fixed number of params, but macros can take a variable number
    • Macros are expanding before compilation, so they can do things like impl'ing traits
    • Macros must be defined above where they're used, whereas functions can be defined anywhere
*/

use hello_macro_derive::HelloMacro;

#[macro_export] // Macro should be available when this crate that defines it is brought into scope.
macro_rules! vec { // The name of this macro is `vec`
    // Pattern of Rust code that this macro is looking for.
    //  Outer set of () encompass the pattern. $ declares a macro variable that contains the matching Rust code.
    //  Next set of () that capture values that match the pattern w/in. $x:expr matches any Rust expression and gives it the name $x.
    //  The , after the () indicates the a comma char must appear between each instance of the code that matches the $x:expr.
    //  The * specifies that the patten matches 0 or more of whatever precedes it.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x); // Push the code found in $x onto the temp_vec
            )* // This $()* pattern generates the code inside of it for each time the pattern matches.
            temp_vec // Return the temp vec
        }
    };
}

fn main() {
    // Declaratibe macros w/ macro_rules!
    {
        let v = vec!(1, 2, 3); // The $x pattern matches 3 times with 1 2 and 3.
        /* The code effectively being run by the macro is:
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
        */

        println!("{v:#?}");
        println!();
    }

    // Procedural macros for generating code from attributes
    {
        // The 2nd form of macros is the prodecural macro. It accepts some code as an input, operates on it, and produces output code.
        // When creating procedural macros, definitions must be in their own crate w/ a special crate type.

        /* Proc macros are defined as such:
        
        use proc_macro;

        #[some_attribute]
        pub fn some_name(input: TokenStream) -> TokenStream {
        }

        The source code the macro is operating on in the input. The code the macro produces is the output.
        */
    }

    // Writing a custom #[derive] macro
    {
        use hello_macro::HelloMacro;

        #[derive(HelloMacro)]
        struct Pancakes;

        Pancakes::hello_macro();
    }

    // Attribute-like macros
    {
        /*
        Arrtibute-like macros are similar to custom #[derive] macros, but instead of
        generating code for the derive attribute, they allow you to create new attributes.

        Derive only works for structs and enums. Attributes can be applied to other things like functions.

        i.e. say we have an attribute named route that annotates fuctions when using a web app framework.
        #[route(GET, "/")]
        fn index() {
            ...
        }

        This #[route] attribute would be defined by the framework as a proc macro.
        The signature of the definition would look like
        #[proc_macro_attribute]
        pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
            ...
        }

        The 1st param is for the contents of the attribute, the `GET, "/"` part.
        The 2nd param is the body of the item #[route] is attached to, index() in this case.
        */

        // Otherwise, attribute-like macros work the same way as custom #[derive] macros.
    }

    // Function-lke macros
    {
        /*
        Function-like macros look like function calls.
        
        Like declarative macros, they can take an take a variable number of args.
        However, declarative macros can only be defined w/ the pattern matching syntax.

        Function-like macros, like the other proc macros, takes in and returns a TokenStream.

        i.e. an sql! macro that parses the sql inside it an sytnax check it.
        let sql = sql!(SELECT * FROM posts WHERE id=1);

        This requires more complex processing than declarative macros are capable of.

        sql! would be defined like
        #[proc_macro]
        pub fn sql(input: TokenStream) -> TokenStream {
            ...
        }
        */
    }
}
