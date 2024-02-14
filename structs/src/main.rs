#[derive(Debug)]
struct User{
    username: String,
    email: String,
    active:bool,
}

fn main() {
   let user1 : User = User{
    email: String::from("aakashmallik8686@gmail.com"),
    username: String::from("aakash"),
    active:true,
   };

//    let user2: User = create_user(
//     email: String::from("aamanMallik@gmail.com"), 
//     username: String::from("aaman")
// );

   let user3: User=User{
        email: String::from("aak123@gmail.com"),
        username: String::from("aah"),
        active: false
    };

    //tuple structs

    struct Colour(i32, i32, i32);
    struct Point(i32,i32,i32);

    println!("user1: {:#?}", &user1);
    // println!("user2: {:#?}", user2);
    println!("user3: {:#?}", &user3);
}


fn create_user(email: String, username: String) -> User{
    User{
        email:email,
        username:username,
        active:true
    }
}