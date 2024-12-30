#[derive(Debug)]
//structs are similar to tuple to hold multiple related values of diff datatypes but the keys can be named
// make it more flexible and order will not be important.
// Structs can also store string references but require lifetime-parameter to be defined
struct User {
    active:bool,
    username:String,
    sign_in_count:u64,
}

//tuple struct
struct Color(i32,i32,i32);

fn main() {

//Define a struct
let user1 = User{
    active:true,
    username:String::from("x123"),
    sign_in_count:1,
};
//access elements in struct with . notation
println!("{}",user1.username);

//Creating struct instances from another instance using struct update
let user2 = User{
    active:false,
    ..user1
};
// Now user1 is not fully usable because the String type from user1 was moved to user2 
// but the other Stack datatypes are still usable like active and sign_in_count.
println!("{}",user2.active);

//tuple struct
let _black = Color(0,0,0);

}