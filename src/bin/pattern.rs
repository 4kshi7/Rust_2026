use std::io;

fn main(){
    println!("Enter width of the pyramid  ");  
    let mut input_var = String::new();
    io::stdin().read_line(&mut input_var).expect("Read line failed");
    
    let num:u8 = match input_var.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number");
            return;
        }
    };

    for _i in 0..num+1{
        for _j in 0.._i{
            print!("*");
        }
        println!();
    }
}

