use std::io;

fn main() {
    let mut s1 = String::from("hello");
    // the pointer,len,capacity is copied but not the data itself
    let s2 = s1;
    //After this stage s1 goes out of scope, because 2 items will try to free memory(double free error)
    //so rust makes s1 go out of scope.
    // println!("{s1}");

    let mut s3: String = String::from("hello");
    //clone
    let s4 = s3.clone();
    println!("{s3},{s4}");
    // integers known in size are stored on stack and easy to copy.
    let x = 5;
    let y = x;
    println!("{x}");

    //references
    let s6 = String::from("hello");
    let len = cal_len(&s6);
    println!("The length of {s6} is {len}");

    //dangling reference fails to compile
    // let some = dangle();

    // no dangle
    let some = no_dangle();
}
fn cal_len(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String{
//     let s = String::from("value");
//     &s // s is deallocated at the end of this function so the reference points to something that was dropped.
// }

fn no_dangle() -> String {
    let s = String::from("value");
    s // return the string itself and not the reference hence ownership is moved.
}
