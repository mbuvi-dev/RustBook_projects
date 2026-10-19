fn main() {
    // this concept in java is called shallow copy.
// but in rust does not allow shallow copy. It uses the concept of ownership. So when we assign s1 to s2, s1 is no longer valid. This is called move semantics in rust. So we cannot use s1 after it has been moved to s2.
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");

}
