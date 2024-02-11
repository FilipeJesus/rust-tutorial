fn main() {
    learning_if_basics(3);
    if_in_a_statement();
    returning_values_from_loops();
    loop_labels();
    while_loops();
    for_loops();
}


fn learning_if_basics(x: i32) {

    // In Rust the condition must evaluate to a bool
    // Nones and non empty lists cannot be used like
    // in python.
    if x % 4 == 0 {
        println!("number is divisable by 4");
    } else if x % 3 == 0 {
        println!("number is divisable by 3");
    } else if x % 2 == 0 {
        println!("number is divisable by 2");
    } else {
        println!("number is not divisable by 4, 3 or 2");
    }

    // Using a lot of elif's can be messy, as previously
    // explored rusts branch construct 'match' can be useful
    // in those casses.
}

fn if_in_a_statement() {
    let condition = true;
    // if is an expression so it can be used on the right of an
    // assignment.
    // The number variable is bound to a value depending on the
    // if expression. the types of the returned values cannot be
    // mismatched.
    let number = if condition {5} else {6};
    println!("The value of the number is: {number}");
}

fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        // the code block in a loop does not have it's own scope
        // at the end of the block the counter variable will
        // contain the value 10
        counter += 1;

        if counter == 10 {
            // loop is an expression it returns the value of
            // the expression defined after the break command.
            break counter *2;
        }
    };

    println!("The result is {result}");
    println!("The counter is {counter}");
}

fn loop_labels() {
    // you can assign label names to loops in order to disambiguate
    // between multiple loops

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
                // specify the label on the break to stop a specific loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn while_loops() {
    // while facilitates conditional looping.
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // getting a range
    for element in (1..4).rev() {
        println!("{element}!");
    }
    println!("LIFTOFF")
}