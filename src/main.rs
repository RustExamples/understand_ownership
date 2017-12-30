fn main() { // "s1" not valid here
    let s1 = "hello"; // "s1" comes into scope
                      // "s1" is of know size and so allocated in stack
                      // "s1" can't be mutated
    
    println!("s1 is {}", s1); // "s1" valid here

    // "s2" not valid here

    let mut s2 = String::from("hello"); // "s2" comes into scope
                                        // "s2" is of unknown size and stored in heap
                                        // "s2" can be mutated

    s2.push_str(", world"); // "s2" valid here

    println!("s2 is {}", s2); // "s2" valid here
} // "s1" goes out of scope and no `drop` is called
  // "s2" goes out of scope and `drop` is called
