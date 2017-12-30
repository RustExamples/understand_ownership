fn main() { // "s1", "s2" & "s3" invalid here
    let s1 = gets_ownership(); // "s1" gets ownership from function
    let s2 = String::from("hello"); // "s2" comes into scope and stored on heap
    let s3 = gives_and_takes_back_ownership(s2); // "s2" `move` to function and invalidated
                                                 // "s3" gets ownership from function

    println!("s1 is {}", s1); // "s1" is valid here
    // println!("s2 is {}", s2); // Throws compile time error
    println!("s3 is {}", s3); // "s3" is valid here

} // "s1" & "s2" goes out of scope and `drop` called

fn gets_ownership() -> String { // "s" is invalid here
    let s = String::from("hello");

    s // "s" `move` to calling function
}

fn gives_and_takes_back_ownership(s: String) -> String { // "s" comes into scope and takes ownership
    s // "s" `move` to calling function
}