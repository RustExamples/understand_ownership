fn main() {
    let s = "hello, world"; // String literal directly stored in binary
                            // "&str" points to specific point in binary
                            // "s" -> "&str" (immutable slice)
                            // So, String literals are immutable by default
}