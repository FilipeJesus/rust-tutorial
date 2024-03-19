pub mod aggregator {
    pub mod libs {}
}

use std::cmp::PartialOrd;
use aggregator::libs::{Summary, Tweet};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
}

// The largest function is generic over some type T,
// and it takes a slice of T values as its parameter
// The largest function will return a reference to a
// value of type T. This function has a single type parameter T,
// and the parameter list for the function is a slice of type T.
// The generic type T must have the PartialOrd trait, this is
// because the values int eh slice must be compared to one
// another.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in list {
        if value > largest {
            largest = value;
        }
    }

    largest
}

// Structs and their methods can also user generics
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Traits can also be used with generics
// The Summary trait is defined in the aggregator crate
// and is used by the Tweet struct
// This function taks a reference to a type that implements
// the Summary trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}