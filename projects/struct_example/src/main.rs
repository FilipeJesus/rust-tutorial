fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (width1, height1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    // using struct methods
    println!(
        "The area of the rectangle is {} square pixels",
        rect2.area()
    );
}

// using individual area and height parameters
// This function is correct, however by supplying
// width and height seperatly there is no garantee
// that the two numbers are actually related to 
// one another
fn area(width: u32, height:u32) -> u32 {
    width * height
}

// user tuples can solve the above problem
// using tuples we can now garantee that the two values
// are related, however this function is not a lot less
// readable. It is not clear what is stored in the
// first and second index, which while not being a problem
// in a simple function like this, it could be an issue
// in a future function.
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Using struct methods
// methods are similar to functions but are defined within the
// context of a struct (or enum or trait) and their first parameter
// is always self, which represetns the instance of the struct the method
// is being called on

// impl block is used to start defining methods belonging to a struct.
// the functions within are also called associated function beacuse
// the are all associated witht he type numbed after the impl

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// here we can defined self as a reference to the instance of type rectangle,
// but methods ca take ownership of self, borrow self immutably or borrow self
// mutably (&mut self) just like any other parameter.

// it is possible to create associated dunctions that don't have self as
// their first parameters (and are thus not methods) because they don;t need an
// instance of the type towork with. This is how String::from() works under the hood.
// these are oftern used as contructos that will return a ner instance of the struct.
// This is done by returns a object of type `Self`

// you can have multiple impl blocks, although there is not real reason to.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}