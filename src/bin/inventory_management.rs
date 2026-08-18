#[derive(Debug, Clone)]
struct Product {
    name: String,
    category: String,
    quantity: u32,
    price: u32,
}

fn analyze_inventory(data: &str) -> Result<(Vec<Product>, u32, Product, u32, u32), String>{

    let mut prod_data:Vec<Product> = Vec::new();
    for line in data.lines(){
        let mut values = line.split(',');


        //we have to safeparse each one of these variables
        //next with either produce some or none in case of string and for integer parsing err or Ok

        let name = match values.next(){
            Some(value) => value,
            None => return Err("Invalid formatting".to_string()),
        };
        
        let category = match values.next(){
            Some(value) => value,
            None => return Err("Invalid formatting".to_string()),
        };


        //let quantity:u32 = values.next().unwrap().parse().unwrap();
        let quantity_str = match values.next(){
            Some(value) => value,
            None => return Err("Invalid formatting".to_string()),
        };

        let quantity = match quantity_str.parse(){
            Ok(value) => value,
            Err(_) => return Err("Invalid formattingcfor quantity, expected a number".to_string()),
            
        };


        //let price:u32 = values.next().unwrap().parse().unwrap();
        let price_str = match values.next(){
            Some(value) => value,
            None => return Err("Invalid formatting".to_string()),
        };

        let price = match price_str.parse(){
            Ok(value) => value,
            Err(_) => return Err("Invalid formatting for price, expected a number".to_string()),
            
        };


        let product = Product {
            name: name.to_string(),
            category: category.to_string(),
            quantity,
            price,
        };

        prod_data.push(product);
    }

    let mut highest_value: u32= 0;
    let mut total_value: u32 = 0;
    let mut highest_value_index: usize = 0;
    let mut total_items: u32 = 0;

    for (i, data) in prod_data.iter().enumerate(){
        let value:u32 = data.quantity * data.price;

        total_value += data.quantity * data.price;
        total_items += data.quantity;
        
        if value > highest_value{
            highest_value = value;
            highest_value_index = i;
        }
         
    }

    let highest_selling_prod: Product = prod_data[highest_value_index].clone();


    Ok((prod_data, highest_value, highest_selling_prod,total_value, total_items))

}

fn main(){
    let data: &str = "Laptop,Electronics,5,75000
Mouse,Electronics,20,1200
Keyboard,Electronics,10,2500
Chair,Furniture,8,5000
Desk,Furniture,3,12000
Mouse,Electronics,5,1200";

    println!("{:?}",analyze_inventory(data));

//ProductName,Category,Quantity,Price
}


// Output ===============================
/*
Ok((
    [
        Product { name: "Laptop", category: "Electronics", quantity: 5, price: 75000 },
        Product { name: "Mouse", category: "Electronics", quantity: 20, price: 1200 },
        Product { name: "Keyboard", category: "Electronics", quantity: 10, price: 2500 },
        Product { name: "Chair", category: "Furniture", quantity: 8, price: 5000 },
        Product { name: "Desk", category: "Furniture", quantity: 3, price: 12000 },
        Product { name: "Mouse", category: "Electronics", quantity: 5, price: 1200 }
    ],
    51,
    Product { name: "Laptop", category: "Electronics", quantity: 5, price: 75000 },
    506000
))
*/