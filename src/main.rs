fn main() {
    let s = String::from("hello world");
    let literal = "hello world";

    let word = first_word(&s);
    let word = first_word(&s[..5]);
    // String literals are *slices* too
    let word = first_word(literal);
    let word = first_word(&literal[..5]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}