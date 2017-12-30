fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1); // &s1 creates a reference
                                        // reference `immutable` like variables

    println!("Length of {} is {}", s1, length); // "s1" valid here

    change(&s1);
} // "s1" goes out of scope and is dropped

fn calculate_length(s: &String) -> usize { // "s" reference to String
                                           // "s" `borrow` from "s1"
                                           // "s" doesn't own
    s.len()
} // "s" goes out of scope and nothing is dropped

fn change(s: &String) { 
    // s.push_str(", world"); // Compile time error trying to modify immutable reference
}