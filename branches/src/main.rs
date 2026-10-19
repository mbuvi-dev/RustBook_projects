// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


// Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean
// if the condition is not a boolean then there will be a compile time error. For example, the following code will not compile:
// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }
/* if you run the above we will get the following error:
error[E0308]: mismatched types
  --> src/main.rs:15:8
   |
15 |     if number {
   |        ^^^^^^ expected `bool`, found integer
*/


// multiple condition using else if
/*
fn main(){
    let number = 7;

    if number % 4 == 0{
        println!("number is divisible by 4");
    }
    else if number % 3 == 0{
        println!("number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

// Using if in a let Statement
// fn main() {
//     let condition = false;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }


// Rust has three kinds of loops: loop, while, and for  
//  So no Do while loop in Rust. The loop keyword creates an infinite loop. You can use the break keyword to stop the loop, and the continue keyword to skip over one iteration of the loop and start the next iteration.


fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}