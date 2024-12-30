fn main() {
    let x = String::from("faith can move mountains");
    let one = first_word(&x);
    println!("1,{one}");

    //string literals are already string slices.
    let y = "faith is love";
    let two = first_word(y);
    println!("2,{two}");

    //references to string slices also works

    let three = first_word(&x[..9]);
    println!("3,{three}")
}

fn first_word(s: &str) -> &str {
    //type of string slice is &str
    //get bytes from string
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        } //check if space,then return the the slice upto the index
    }
    &s[..]
}
