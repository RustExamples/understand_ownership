fn main() { // "s1" and "s2" invalid here
    let s1 = String::from("hello"); // "s1" comes into scope and is stored on heap
    let s2 = s1.clone(); // "s1" is deep copied to "s2"
                         // "s1" still valid here

    println!("s1 is {}, s2 is {}", s1, s2); // "s1" and "s2" valid here
}  // "s1" & "s2" goes out of scope and `drop` is called