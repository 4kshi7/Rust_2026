use std::io;

fn push_str_akshit(s:String) -> String {
    
    println!("Enter the string you want to append");
    let mut string_two = String::new();
    io::stdin().read_line(&mut string_two).expect("Failed to read line");
    return s+&string_two; 
}

fn main() {
    let input_string = String::from("Hello");
    let input_string_two = push_str_akshit(input_string);
    println!("{}", input_string_two);

        
}   