use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    // the panic! macro will cause the program to crash
    // panic!("crash and burn")

    // panic can be combined with the Result enum
    // The error is a variant of io::Error, which is a struct provided
    // by the standard library. This struct has a kind method that we can
    // call and match it against the expected ErrorKind variant.
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };

    // If you do not want to handle the error, you can use the unwrap or
    // expect methods. These methods are shortcuts for the match expression
    // we used above. If the Result value is the Ok variant, unwrap will
    // return the value inside the Ok. If the Result is the Err variant,
    // unwrap will call the panic! macro for you. The expect method is
    // similar to unwrap, but it allows you to specify the panic! message.
    let _greeting_file = File::open("hello.txt").unwrap();
    let _greeting_file = File::open("hello.txt").expect(
        "Failed to open hello.txt"
    );

    // Propagating errors
    let _username = read_username_from_file()
        .expect("Failed to read username from file");

    // The ? operator can be used to propagate errors
    let _username = read_username_from_file_short()
        .expect("Failed to read username from file");
}

// Propagating errors
// When you are writing a function that calls another function that
// might fail, instead of handling the error within the function, you
// can return the error to the calling code so that it can decide what
// to do.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

    // The ? operator can be used to propagate errors
    // The ? operator can only be used in functions that have a return
    // type of Result or Option. If the value of the Result is Ok, the value inside
    // the Ok will be returned from the expression. If the value is Err,
    // the Err will be returned from the whole function. When using the ? operator,
    // with Option, the Some variant will be returned from the expression, and the
    // None variant will be returned from the whole function.
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}