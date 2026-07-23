struct User {
    name: String,
    email: String,
    active: bool,
    id: u32,
}

fn main() {
    let users = vec![
        User {
            name: String::from("Akshit"),
            email: String::from("akshit@gmail.com"),
            active: true,
            id: 1,
        },
        User {
            name: String::from("Rahul"),
            email: String::from("rahul@gmail.com"),
            active: false,
            id: 2,
        },
        User {
            name: String::from("Raman"),
            email: String::from("raman@gmail.com"),
            active: true,
            id: 3,
        },
        User {
            name: String::from("Sid"),
            email: String::from("sid@gmail.com"),
            active: true,
            id: 4,
        },
    ];
    
    for i in &users{
        println!(
            "Name: {}\nEmail: {}\nActive User: {}\nUser ID: {}\n",
            i.name,
            i.email,
            i.active,
            i.id
        );
    }
 
}