// fn main() {
//     println!("Hello, world!");
//     let mut x= 5;
//     println!("The value of x is : {}",x);
//     x =6;
//     println!("The value of x is : {}",x);
// // }

// I learned the differnt variables on the rudt.
/*interger types
i8, i16, i32, i64, i128, isize --signed integer types thus mean that they can be positive or negative numbers
u8, u16, u32, u64, u128, usize --unsigned integer types thus mean that they are positive numbers only
floating point types
f32, f64
boolean type
bool = true or false
character type 
char = rep resents a single character, like 'a', 'α', or '∞'

*/

// fn main() {
//     let x = 2.0; // f64
//     println!("The value of x is : {}",x);
//     let y: f32 = 3.0; // f32
//     println!("The value of y is : {}",y);
// }

/* i also learned about the compound types in rust
tuple type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in
it is not mutable, but you can destructure a tuple into its individual parts and access them by their index or by pattern matching.
we have to use tup=  to create a tuple and then we can access the values in the tuple by using the dot operator and the index of the value we want to access.

array type
An array is a collection of values of the same type. Arrays in Rust have a fixed length
and cannot grow or shrink in size. The length of an array is part of its type,
so arrays of different lengths are considered different types. Arrays are useful when you want your data allocated on the stack rather than the heap, and when you want to ensure that your data is stored cont
*/

/* to acess the values on the tuple
we declare a another variable and assign it to the tuple and then we can access the values in the tuple by using the dot operator and the index of the value we want to access.
*/
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {x}");
// }

// arrays
/*
ararys are different from other languanges like in js array are muatable but in rust the leng is fixed.
also the way we access the values in the array is different from other languanges like in js we use the index of the value we want to access but in rust we use the dot operator and the index of the value we want to access.

*/
// example the code below is an example of how to access the values in an array in rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}