struct User{
    name: String,
    email: String,
    active: bool,
    id: u32
}

fn main() {
    println!("Hello, world!");
    //struct
    let user1 = User { 
        name: String::from("Akshit"),
        email:String::from("akshit@gmail.com"),
        active:true,
        id:1
    };

    println!("Name : {}\n Email : {}", user1.name);
}
