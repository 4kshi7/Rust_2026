use std::collections::HashMap;
#[derive(Debug,Clone)]
struct Order{
    customer: String,
    product: String,
    category: String,
    quantity: u32,
    price: u32,
}
//parsing functions
fn parse_u32(val: Option<&str>, field: &str) -> Result<u32, String> {
    let val = match val {
        Some(val) => val,
        None => return Err(format!("Missing {}", field)),
    };

    match val.parse::<u32>(){
        Ok(val) => Ok(val),
        Err(_) => return Err(format!("Invalid {}, expected a number", field)),
    }
}


fn parse_string(val: Option<&str>, field: &str) -> Result<String, String> {
    match val {
        Some(val) => Ok(val.to_string()),
        None => return Err(format!("Missing {}", field)),
    }
}

fn analyze_orders(data: &str) -> Result<(Vec<Order>, HashMap<String,u32>, HashMap<String,u32>, String, Order), String>{
    let mut order_vector: Vec<Order> = Vec::new();

    for line in data.lines(){
        let mut values = line.split(',');
        let customer = parse_string(values.next(), "customer")?; 
        let product = parse_string(values.next(), "product")?; 
        let category = parse_string(values.next(), "category")?;
        let quantity = parse_u32(values.next(), "quantity")?; 
        let price = parse_u32(values.next(), "price")?; 

        let order_data = Order {
            customer,
            product,
            category,
            quantity,
            price,
        };

        order_vector.push(order_data);    
    }

    //customer hashmap
    let mut customer_hash: HashMap<String,u32> = HashMap::new();
    let mut category_hash: HashMap<String,u32> = HashMap::new();
    
    
    for data in &order_vector{
        *customer_hash.entry(data.customer.clone()).or_insert(0) += data.quantity * data.price;
        *category_hash.entry(data.category.clone()).or_insert(0) += data.quantity;
    }

    //highest spender 
    let mut highest_spender:String = String::new();
    let mut highest_rev:u32 = 0;
    

    for (key,val) in &customer_hash{
        if highest_rev < *val{
            highest_rev = *val;
            highest_spender = key.clone();
        } 

        // println!("{highest_spender}");
    }

    //most expensive order
    let mut expensive_order_index:usize = 0;
    let mut expensive_order:u32 = 0;
    for (i, data) in order_vector.iter().enumerate() {
        if data.quantity * data.price > expensive_order{
            expensive_order = data.quantity * data.price;
            expensive_order_index = i;
        }
        // println!("{}",expensive_order);
    }

    let expensive_order_vec = order_vector[expensive_order_index].clone();



    Ok((order_vector, customer_hash, category_hash, highest_spender, expensive_order_vec))

}

            fn main(){

    let data: &str = "alice,Laptop,Electronics,2,75000
bob,Mouse,Electronics,3,1200
alice,Keyboard,Electronics,1,2500
charlie,Chair,Furniture,2,5000
bob,Laptop,Electronics,1,75000
alice,Chair,Furniture,1,5000
charlie,Desk,Furniture,1,12000";

    println!("{:#?}", analyze_orders(data));

}

/*
(
    Vec<Order>,
    HashMap<String, u32>,   // customer -> total revenue
    HashMap<String, u32>,   // category -> total quantity
    String,                 // highest spending customer
    Order                   // most expensive order
)
*/