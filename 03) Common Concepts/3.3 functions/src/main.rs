fn main() {
    print_labeled_measurement(5, 'h');

    expressions_vs_statements();

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");
    let x = plus_one_ret(x);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    // parameter types must be declared
    println!("The measurement is: {value}{unit_label}");
}

fn expressions_vs_statements() {
    /*
        Statements are instructions that perform some action and do not return a value.
            i.e. let, fn
            - statements technically return (), the unit type
        Expressions evaluate to a resultant value.
            i.e. 5 + 6, fncall()
    */

    // Scopes are also expressions
    let y = {
        let x = 3; // statements end w/ semicolons
        x + 1 // expressions do not include semicolons
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    // specify return type of func w/ -> type
    // return value of func = final expression
    5 // 5 in this case
}

fn plus_one(x: i32) -> i32 {
    x + 1 // compiler error if a ; is added, as the function no longer evaluates to an expression
}

fn plus_one_ret(x: i32) -> i32 {
    return x + 1; // return statement can be used to force return w/ expression
}
