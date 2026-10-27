fn main() {
    println!("Hello, world!");
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// struct USer{
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:u64,
// }
// we dont need to use the struct keyword each time so we can just use the name of what the object is
fn build_user(email:String,username:String)->User{
    let mut user1= User{
        active:true,
        username:username,
        email:email,
        sign_in_count:1,
        
    }
    println!("The user is: {:?}", user1);
}