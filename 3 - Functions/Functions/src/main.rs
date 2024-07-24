// This is an example of a summatory function using the Rust language.
// The parameters specify the type of each variable and -> indicates the return type of the function.
// In this case, it returns an integer of 32 bits.

fn add(a: i32, b: i32) -> i32 {
    a + b // No need for a semicolon here because it's an expression
}

// This one is a void return function example.

fn kinda_void(a: String) {
    println!("{}", a);
}

fn main() {
    println!("{:?}", add(2, 2)); // returns 4

    let this_example = add(9, 2);
    println!("{:?}", this_example); // returns 11

    kinda_void("\nThis is a String!".to_string()); // returns the written string
}
