fn main() { // "s" & "x" invalid here
    let s = String::from("hello"); // "s" comes into scope and stored on heap

    takes_ownership(s); // "s" `move` into function and is invalidated

    // println("s is {}", s); // Throws compile time error

    let x = 5; // "x" comes into scope

    makes_copy(x); // "x" copied to function

    println!("x is {}", x); // "x" still valid here
} // "x" goes out of scope and no `drop` called

fn takes_ownership(s: String) { // "s" comes into scope and takes ownership
    println!("s is {}", s); // "s" valid here
} // "s" goes out of scope and `drop` is called

fn makes_copy(x: i32) { // "x" comes into scope
    println!("x is {}", x); // "x" valid here
} // "x" goes out of scope and no `drop` called