fn main() {
    // vectors are made using Vec<t>

    // Create a new vector
    let mut v_mut: Vec<i32> = Vec::new();

    // You can also use the vec! macro to create a vector
    let v = vec![1, 2, 3];

    // You can add a value to a vector using the push method
    v_mut.push(5);
    v_mut.push(6);
    v_mut.push(7);

    // You can access elements of a vector using indexing
    let third: &i32 = &v_mut[2];

    // You can also use the get method to access elements
    let third: Option<&i32> = v_mut.get(2);

    match third {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element"),
    }

    // When indexing, you can't access an element that doesn't exist
    // let does_not_exist = &v_mut[100]; // This will cause a panic
    // You can use the get method to avoid this

    // If you update a vector, all references to it are invalidated

    // You can iterate over a vector using a for loop
    for i in &mut v_mut {
        // dereference i to get the value
        *i += 50;
    }

    // You can only store one type of value in a vector, in order to
    // store multiple types, you can use an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
