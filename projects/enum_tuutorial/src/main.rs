fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Cannot do x + y because x and y are different types
    // Need to first check that y is not None and convert
    // Option<T> to type T. This gives confidence to our code.
    // To use the value in y you need to use code which will
    // handle each variant such as the match expression e.g.
    match y {
        None => None,
        Some(i) => Some(x + 1),
    };
    // match can be used with enums to program what a program should
    // do with any variant of a enum, the code above first checks that
    // y is of type Some(i) (i being a variable for the internal value)
    // and runs code in the bracket it it does. If not then it will check
    // if y is of type None, if it is then None is returned.
    // Matches are exhaustive, you must cover all cases of the enum. You
    // can use either the other keyword or the _ placholder if you need a
    // catch all branch of your match expression.
}


// enums give you a way of saying a value is one of a possible set of values

enum IdAddrTuppleStruct {
    V4(u8, u8, u8, u8),
    V6(String)
}


struct Ipv4Addr(
    u8, u8, u8, u8
);

struct Ipv6Addr(
    String
);

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

// you can define these individual structs
struct QuitMessage; // Unit struct
struct MovieMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); // Tuple struct
struct ChangeColorMessage(i32, i32, i32); // Tuple struct

// or you can use a enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can define methods for enums also using impl
impl Message {
    fn call(&self){
        println!("Call internat atribute")
    }
}

// Rust does not have a null value/reference. This is a protective issue
// because in other languages issues usually occur when calling a method on
// a null value. Instead rust has the Option<T> enum
// enum Option<T> {
//     None, Some(T)
// }
// This is included in the rust prelude
// The <T> syntax is a generic type parameter. So <t> means that the Some variant
// of the option enum can hold one piece of data of any type, and that each concrete
// type that gets used in place of T makes the overall Option<T> type a different type.