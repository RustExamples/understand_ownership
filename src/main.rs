fn main() {
    let s1 = String::from("vms"); // "s1" comes into scope

    let r1 = &s1; // "r1" `borrow` "s1"

    let s2 = s1; // compile errorr
    			 // "s1" can't `move` into "s2" because it is borrowed
                 // by "r1"
}