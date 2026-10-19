// fn main() {
//     println!("Hello, world!");

//     where_am_i();
// }
// fn where_am_i() {
//     println!("I am in the chapter 3 functions.");
// }
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

