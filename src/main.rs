fn main() {
	let s = get_string();

    println!("s is {}", s);
}

fn get_string() -> String {
	let s = String::from("hello"); // "s" comes into scope
    
    s // "s" `move` into calling function and is invalidated
} // "s" goes out of scope and no drop required