fn main() { // "x", "y", "s1" & "s2" invalid here
    let x = 5; // "x" comes into scope
    let y = x; // copy "x" to "y"

    println!("x is {}, y is {}", x, y); // "x" is still valid here
                                        // "x" is on stack
                                        // integer types have `Copy` trait

    let s1 = String::from("hello"); // "s1" comes into scope & stored on heap
    let s2 = s1; // "s1" `move` into "s2" and is invalidated
                 // "s1" invalid here

    //println!("s1 is {}", s1); // Throws compile time error
    println!("s2 is {}", s2); // "s2" valid here
} // "x" and "y" goes out of scope and no `drop` is called
  // "s2" goes out of scope and `drop` is called