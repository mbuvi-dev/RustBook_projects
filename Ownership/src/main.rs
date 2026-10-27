fn main (){
    first_word(&String::from("hello world"));
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
    println!("The first word is: {}", &s[0..first_word(s)]);
}
