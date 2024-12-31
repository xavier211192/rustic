
fn answer()-> i32 {
    let x: i32 = 42;
    x
}

fn main() {
    let val = answer();
    println!("{}",val)
}

#[test]
fn check_validity() {
    assert_eq!(answer(), 42)
}