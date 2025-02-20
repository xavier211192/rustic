//derive trait to allow displaying structs
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}


fn main() {
    let rect1 = Rectangle{
        width:12,
        height:12,
    };
    println!("{:#?}",rect1); //{:#7} to make it more readable
    println!("Area is {}",area(&rect1));
}

fn area(rec:&Rectangle) -> u32 {
    rec.width * rec.height
}