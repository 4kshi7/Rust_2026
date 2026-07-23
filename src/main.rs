use std::f64::consts::PI;

struct User {
    name: String,
    email: String,
    active: bool,
    id: u32,
}

struct Rectangle {
    height: u32,
    width: u32,
    id: u32,
}

enum Shapes{
    Rectangle(f64,f64),
    Circle(f64),
    Square(f64),
}

fn calculate_area(shape: Shapes) -> f64 {
    match shape{
        Shapes::Rectangle(w,h) => w * h,
        Shapes::Circle(r) => PI * r * r,
        Shapes::Square(s) => s*s,
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.height + self.width);
    }
}

fn main() {

    //structs
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
 
    //implementation
    let rectangles = vec![
        Rectangle {
            height: 10,
            width: 20,
            id: 1,

        },
        Rectangle {
            height: 20,
            width: 20,
            id: 2,
        },
    ];

    for i in &rectangles {
        println!("Area of rectange id {} is {}", i.id, i.area());
        println!("Perimeter of rectangle id {} is {}", i.id, i.perimeter());
    }

    //enums

    let circle = Shapes::Circle(7.0);
    let square = Shapes::Square(5.0);
    let rectange = Shapes::Rectangle(10.0,10.0);

    println!("Circle : {}\nSquare : {}\nRectangle : {}", calculate_area(circle), calculate_area(square), calculate_area(rectange));

}