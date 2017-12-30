fn main() {
    let mut s1 = String::from("hello"); // make "s1" mutable

    change(&mut s1); // create "mutable" reference of "s1"

    println!("s1 is {}", s1);

    // compile time error 
    // atmost one mutable reference to "s1" possible
    // let r1 = &mut s1;
    // let r2 = &mut s1;

    {
        let r1 = &mut s1;
    } // "r1" goes out of scope and returns what it borrowed

    let r2 = &mut s1; // valid usage here

    // compile time error 
    // mutiple immutable references possible
    let r1 = &s1;
    let r2 = &s1;
    // mixing mutable and immutable references in same scope
    let r3 = &mut s1;

} // "s1" goes out of scope and is dropped

fn change(s: &mut String) {  // accept a "mutable" reference
     s.push_str(", world"); 
}