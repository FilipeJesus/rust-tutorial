fn main() {

    // By default all variables are immutable, you can
    // make then mutable by using `mut`

    // Mutable Variables
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    // You cannot use `mut` with constants, they are always
    // immutable and you must provide the data types.
    // They can be declared at any scope including global.
    // They cannot be set as the result of computation which
    // has to be calculated at runtime, there are exceptions
    // for basic evaluations like below which can be evaluted
    // during compile time.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    // Overwriting existing variables, the second variable is what
    // the compiler will see when you use the name of the varaible.

    let x = 5;
    let x = x + 5;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
