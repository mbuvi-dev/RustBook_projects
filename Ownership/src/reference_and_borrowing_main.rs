// fn main() {
//     // this concept in java is called shallow copy.
// // but in rust does not allow shallow copy. It uses the concept of ownership. So when we assign s1 to s2, s1 is no longer valid. This is called move semantics in rust. So we cannot use s1 after it has been moved to s2.
// // in Rust shallow copy is know as a move. 

// //     let s1 = String::from("hello");
// //     let s2 = s1;

// //     println!("{s1}, world!");

// // }

// code that ensures move in rust
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// using the mutabale reference
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{s}");
//      //the new s value is now "hello,chwani world" and the scope is here
//     let s1 = &mut s;//we can modify the value of s1 since now s scope we already chnage and in chnage function it is out of scope. So we can use s1 to modify the value of s.
//     s1.push_str(",mbao world");
//     println!("{s1}"); //but now s1 is out of scope and we cannot use it anymore. So we can use s to modify the value of s.
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(",chwani world");
// }

/*
// we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
// example of using curly brackets to create a new scope
fn main() {
    let mut s = String:: from ("hello");
    {
        let s1 =&mut s;
        s1.push_str(",chwani world");
        println!("{s1}");
    }// s1 is out of scope here, so we can make a new reference with no problems.
    let s2 = &mut s;
    s2.push_str(",mbao world");
    println!("{s2}");

    immutable_reference();
}

// but w ecan have multiple immutable ref to variable but not mutable ref to variable and immutable ref to variable at the same time.
fn immutable_reference() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r5 = &mut s; // big problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // the problem is when we try to mutate the variable example. 
    let r4 = &mut s; // big problem
}
    */
/* Dangling reference. 
by definition a dangling reference is a pointer that references a memory location that might have been deallocated.
for example we have a refrence to a variable that is out of scope. 
for example: 
*/
// fn main(){
//     let reference_to_nothing = dangle();

// }
// // the main function is calling dangle function and passes a pram that is not yet defined.


// fn dangle () -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope and is dropped here, so the reference is dangling.

// /* so we get the error
// | fn dangle () -> &String {
//    |                 ^ expected named lifetime parameter
// */