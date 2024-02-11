fn main() {
    another_function(1, 'h');
    using_expressions(5);
    let x = plus_one(6);
    println!("The value of 6 plus one is: {x}")
}

// Snakecase used for function names
// parameters can be defined in the perenthesis
// assign types to the paramers
fn another_function(value: i32, unit_label: char) {
    println!("The value of x is: {value}{unit_label}.");
}

// functions are constructed as a series of statements and expressions
// Rust is an expression based language. The meaning of these are:
//    - Statements are instructions that perform some action and do
//      not return a vlaue. 'let y = 6;' is a statement as it does not
//      return a value.
//    - Expressions evaluate to a resultant value. '5 + 6' is an expression.
fn using_expressions(x: i32) {
    let y = {
        let z = x;
        // This is an expression, they do not include ending semicolons
        z + 1
    };
    println!{"The value of y is: {y}"}
} 

// functions which return a value must define the type of the returned value
fn plus_one(x: i32) -> i32 {
    // This is an expression
    x + 1
}