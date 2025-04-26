fn main() {
    /* Loops continuously. Escape with ^C.
    loop {
        println!("again!");
    } */

    ret_val_from_loop();
    loop_labels();
    while_loop();
    for_loop_index();
    for_loop_element();
    for_loop_range();
}

fn ret_val_from_loop() {
    let mut counter = 0;
    let result = loop { 
        counter += 1;

        if counter == 10 {
            break counter * 2; // exit the loop and set result = counter * 2
        }
    };

    println!("The result is {result}\n");
}

fn loop_labels() {
    /*  break & continue by default apply to innermost loop at the point of calling.
        A loop label can be used w/ break & continue to apply to the labeled loop instead
    */


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}\n");
}

fn while_loop() {
    let mut number = 3;

    while number > 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!\n");
}

// Loop over array with index. Risk at runtime of exceeding array bounds, compile time will not catch them.
fn for_loop_index() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!();
}

// Loop over array be element. More elegent, performant, and error-proof
fn for_loop_element() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!();
}

// for loop + range if code needs to run certain # of times.
// range generates all #s in sequence from start # to one before end #
fn for_loop_range() {
    for number in (1..4).rev() { // 1, 2, 3 is reversed
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}