fn main() {
    let height = 12;
    let width = 12;

    println!("Area is {}",area(width,height));
}

fn area(width:u32,height:u32) -> u32 {
    width*height
}