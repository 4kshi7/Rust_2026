use std::f64::consts::PI;
use std::io;

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

    //some basic functions
    println!("{}, {}", is_even(2), is_even(3));
    println!("{}", fibonacci(4));
    let a:u32 = 4;
    println!("Factorial of {} : {}",a, fac(a));
    let s = String::from("Hello world");
    println!("{}", str_len(&s));
    println!("{}", prime_number(7));


    //Taking Input from user in rust

    //string

    //3 Step Pipeline : Read raw bytes into String -> Trim whitespaces & newlines -> Parse into target datatype 
    
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    //Trim new lines (\n) and parse into i32 

    let number:i32 = match input_value.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            println!("Invalid number entered!");
            return;
        }

    };
    println!("{}", number);

    //Reading a Number (i32, u32, f64, etc.)

    let mut input_number = String::new();
    println!("Enter a number");
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number:i32 = input_number.trim().parse().expect("Expected a numeral");
    println!("{}", input_number);

}

//odd even
fn is_even(n:u32) -> bool {
    if n%2 == 0{
        return true;
    }
    return false;
}

//fibonacci
fn fibonacci(n:u32) -> u32 {
    if n==0 || n==1 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

//factorial

fn fac(n:u32) -> u32 {
    if n==0 || n==1{
        return 1;
    }

    return n * fac(n-1);
}

//function that return lenght of a string

fn str_len(s:&str) -> u32 {
    let mut count = 0;
    for _ in s.chars(){
        count = count +1;
    }
    return count;
}

//prime number

fn prime_number(n:u32) -> bool {
    if n==0 || n==1 {
        return false;
    }

    let mut flag = true;

    for i in 2..n{
        if n%i == 0 {
            flag = false;
            break;
        }
    }

    return flag;
}