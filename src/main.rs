fn main() {
    let mut s = String::from("hello world");
    // let s1 = &s[0..5];
    // let s2 = &s[6..11];
    // let s3 = &s[0..]; // end index is length
    // let s4 = &s[..5]; // start index is 0
    // let s5 = &s[..]; // take whole string

    let word = first_word(&s); // Pass "immutable" reference of "s"
    						   // Get "immutable" slice of "s"

    s.clear(); // Compile time error
    		   // "s.clear()" tries to get a "mutable" reference of "s"
               // we can't have "mutable" and "immutable" reference
               // to same data in same scope
}

fn first_word(s: &String) -> &str { // Return "immutable" slice
    let bytes = s.as_bytes(); // Return "immutable" reference

    for (i, &byte) in bytes.iter().enumerate() { // Return (integer, reference)
        if byte == b' ' {
            return &s[..i] // Return "immutable" slice
        }
    }

    &s[..] // Return "immutable" slice
}