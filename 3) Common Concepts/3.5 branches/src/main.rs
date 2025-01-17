fn main() {
    let number = 3;

    if number < 5 {
        // if statements can only take expressions that evaluate to bool type
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    {
        // If is an expression, so it can be used in a let statement
        let condition = true;
        let number = if condition { 5 } else { 6 }; // each arm has to return the same type for the let assignment, i.e. "six" isn't allowed
        println!("The value of number is: {number}");

        // Another way
        let condition = true;
        let number;
        if condition {
            number = 5;
        } else {
            number = 6;
        }
        println!("The value of number is: {number}");
    }
}
