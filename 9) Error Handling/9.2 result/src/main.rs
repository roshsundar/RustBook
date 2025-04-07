/* Summary:
The Result<T, E> enum can be used to handle errors the program can recover from (not crash).
It has 2 variants: Ok(T) (the success case), and Err(E) (the fail case), w/ T and E being generic types.
*/

use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    // Handling a result from a function
    {
        // Return type is Result, with success containing a File and failure containing an Error.
        // File::open() can fail if the file might not exist or we might not have adequate permissions.
        let greeting_file_result = File::open("hello.txt");

        // Handle the Result. In the success case, we grab the file out of the Ok variant.
        // In the case of failure, panic! w/ the Error.
        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
    }

    // Matching on different errors
    {
        // Attempt to open a file. If it fails, create a file

        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() { // The type inside the Err returned by File::open() is an io::Error, which has a kind() method to get a variant describing the type of io error
                ErrorKind::NotFound => match File::create("hello.txt") { // hello.txt doesn't exist yet, so attempt to create it
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"), // create hello.txt fail case
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            }
        };
    }

    // Shortcuts for panic on error
    {
        // unwrap() method implemented on Result that:
        //  if Ok, then get the value inside
        //  if Err, then panic!
        let _greeting_file = File::open("hello.txt").unwrap(); // will panic! if hello.txt doesn't exist

        // expect() does the same thing but the panic! message can be customized
        let _greeting_file = File::open("hello.txt").expect("can't find hello.txt");
    }

    // Propogating errors
    {
        // A function might do something that fails. Rather than handling that in the function,
        // you can return it to the caller so it can decide what to do. This is called propogating the error.

        let _result = read_username_from_file_1(); // At this point can decide what do to with the Result

        // Error propogation is common, so Rust provides the ? operator to make this easier.
        let _result = read_username_from_file_2();

        // More condensed version of the prior function
        let _result = read_username_from_file_3();

        // Where can the ? be used
        // It can be used in any function that returns Result or Option.
        // Can't mix though, i.e using ? on a Result in a function that returns Option, or vice versa.
        let _result = last_char_of_first_line("text");
    }
}


fn read_username_from_file_1() -> Result<String, io::Error> {
    // Return type is Result<T, E> where T = String, and E = io::Error.
    // If the function succeeds, then Ok(String) is the Result, which is the username read from hello.txt
    // If it fails, an Err(io::Error) is the Result, which contains info abt what went wrong.

    // Attempt to open the file. If it works, grab the file. If not, return the Error to the calling function.
    let mut username_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();

    // Call read_to_string() on the file to try reading the contents out to username.
    // If it succeeds, return the username in Ok, if not return an Error in Err.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error>  {
    // The ? operator placed after a Result val will:
    //  If Ok, extract the value inside, and the function will continue
    //  If Err, the function will return to the caller with Err

    // Attempt to get the file hello.txt. On fail, return to the caller w/ Error
    let mut username_file = File::open("hello.txt")?; 

    let mut username = String::new();

    // Attempt to read the file out to username. On fail, return to the caller w/ Error
    username_file.read_to_string(&mut username)?;

    // The priror read_to_string() succeeded and didn't return to caller.
    // Now return the username to caller.
    Ok(username)
}
/*
Error values that have ? called on them go through the from() function, which converts the error type
recieved into an error type of the current function.
This is useful when a function returns one error type to represent all ways of failure for a function.

i.e. we can change the above function to return a custom error OurError (-> Result<String, OurError>). 
If we * impl From<io::Error> for OurError * to construct an instance of OurError from io:Error, 
then ? calls in that function will call from() and convert the errors into OurError automatically.

i.e. File::open("hello.txt")? , open() creates and io::Error that ? then converts into OurError and returns.
*/

// More ergonomic writing of the prior function
fn read_username_from_file_3() -> Result<String, io::Error>  {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // The ? operator placed after a Option val will:
    //  If Some, extract the value inside, and the function will continue
    //  If None, the function will return to the caller with None

    // text.lines().next() attempts to get the first line and returns an Option.
    // Then ? evaluates the Option.
    //  If the str is empty, it will find None and return to the caller w/ that.
    //  If not, then it will extract the text inside Some and continue.
    // chars() gets an iteratior of the extracted str, and last() gets the last char in an Option
    // b/c the first line may be a \n char.
    text.lines().next()?.chars().last()
}