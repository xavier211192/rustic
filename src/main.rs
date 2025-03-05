// Enums give a way to say that a value is one of the possible set of values.
// enumerate all possible variants.

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}
//data can be put directly in each enum variant
//each variant can have different types and amounts of associated data(not possible in struct)
//any kind of data can be put inside enum variant including a struct and enum itself.
let home = IpAddr::V4(127,0,0,1);
let home2 = IpAddr::V6(String::from("::1"));

//complex enum
enum Message {
    Quit,
    Write(String),
    Move{x:i32, y:i32},
    ChangeColor(i32, i32, i32)
}

impl Message{
    fn call(&self){
        //do something
    }
}

let m = Message::Write(String::from("hey"));
m.call();

//Option enum type(std library)
//Rust does not have NULL feature
//Option and its variants are already in scope in prelude.
//  enum Option<T> {
//      None,
//      Some(T),
//      }
let some_number = Some(1); //variant of Option type
let some_char = Some("h");
let absent_number: Option<i32> = None; //variant of Option type

//Why is it important
// compiler will not compile this code because Option<T> is a different type.
let x:i8 =5;
let y:Option<i8>= Some(5);
let sum = x+y;
//if a value can possibly be null then opt in using Option<T> so you are forced to handle that case.
//in order to use Option<T> value you should have code that handles all variants of Option.