fn main() {
	let dangling_reference = get_dangling_reference(); // dangling reference
}

fn get_dangling_reference() -> &String { // Returns reference to String
	let s = String::from("hello"); // "s" comes into scope
    
    &s // reference to "s" returned
} // "s" goes out of scope and dropped