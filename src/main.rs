//derive trait to allow displaying struct
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}
//to create method start impl everything in this block will be associated with the Rectangle type(struct)
//these methods are called associated functions
impl Rectangle{
    //method takes arguments and first argument is self which is instance of struct
    fn area(self:&Self)->u32{
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle)->bool {
        self.width > other.width && self.height > other.height
    }

}

fn main() {
    let rect1 = Rectangle{
        width:12,
        height:12,
    };
    let rect2 = Rectangle{
        width:5,
        height:5,
    };
    println!("{:#?}",rect1); //{:#7} to make it more readable
    println!("Area is {}",rect1.area());
    println!("Can rec1 hold rect 2? {}",rect1.can_hold(&rect2));
