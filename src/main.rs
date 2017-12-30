fn main() {
    let mut s = String::from("Hello World"); // "s" comes into scope

    let first_word = get_first_word(&s); // get index of first word

    s.clear(); // "first_word" & "s" are out of sync
               // "first_word" holds 5 but "s" is empty here
}

fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // conver to byte array

    for (i, &item) in bytes.iter().enumerate() { // enumerate returns (int, reference)
        if item == b' ' {
            return i // return if space found
        }
    }

    s.len() // return length of string if no space found
}