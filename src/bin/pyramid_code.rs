use std::io;

fn main() {
    println!("Enter input value");

    let mut input_var = String::new();

    io::stdin()
        .read_line(&mut input_var)
        .expect("Unable to read line");

    let input_num: u32 = match input_var.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid numeral");
            return;
        }
    };

    for i in 0..input_num {
        for _ in 0..(input_num - i - 1) {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }

        println!();
    }
}