use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    status_code: u32,
    response_time: u32,
}

fn analyze_api(data: &str) -> Result<(Vec<Request>, HashMap<String, u32>, HashMap<String, f32>, String, Request), String>{
    let mut request_vec:Vec<Request> = Vec::new();
    
    for line in data.lines(){
        let mut values = line.split(',');

        let method = match values.next(){
            Some(val) => val,
            None => return Err("Invalid method".to_string()),
        };

        let path = match values.next(){
            Some(val) => val,
            None => return Err("Invalid path  ".to_string()),
        };

        let status_code_str = match values.next(){
            Some(val) => val,
            None => return Err("Invalid status code  ".to_string()),
        };

        let status_code: u32 = match status_code_str.parse(){
            Ok(val) => val,
            Err(_) => return Err("Invalid status code  ".to_string()),
        };

        let response_time_str = match values.next(){
            Some(val) => val,
            None => return Err("Invalid status code  ".to_string()),
        };

        let response_time: u32 = match response_time_str.parse(){
            Ok(val) => val,
            Err(_) => return Err("Invalid status code  ".to_string()),

        };

        let api_data = Request{
            method: method.to_string(),
            path: path.to_string(),
            status_code,
            response_time,
        };

        request_vec.push(api_data);

    }


    // request count per endpoint
    let mut endpoint_count:HashMap<String, u32> = HashMap::new();
    let mut total_response:HashMap<String, u32> = HashMap::new();
    let mut avg_response:HashMap<String, f32> = HashMap::new();

    for (_, data) in request_vec.iter().enumerate(){

        *endpoint_count.entry(data.path.clone()).or_insert(0) += 1;
        *total_response.entry(data.path.clone()).or_insert(0) += data.response_time;

    }


    //avg response time per endpoint
    for (endpoint, count) in &endpoint_count{
        //println!("{endpoint} -> {count}");
        let total = *total_response.get(endpoint).unwrap();
        let avg = total as f32 / *count as f32;
        
        avg_response.insert(endpoint.clone(), avg);
    }


    //busiest endpoint
    let mut highest_count:f32 = 0.0;
    let mut busiest_endpoint: String = String::new();
    
    for (endpoint, time) in &avg_response{
        if *time > highest_count{
            highest_count = *time;
            busiest_endpoint = endpoint.to_string();
        }
    }

    //slowest request
    let mut slowest_req_index:usize = 0;
    let mut slow_req:u32 = 0;
    for (i, data) in request_vec.iter().enumerate(){
        if data.response_time > slow_req  {
            slow_req = data.response_time;
            slowest_req_index = i;
        }
    }
    //println!("{} and index:{}", slow_req, slowest_req_index);

    let slowest_request : Request = request_vec[slowest_req_index].clone();

    Ok((request_vec, endpoint_count, avg_response, busiest_endpoint, slowest_request))
}


fn main(){
    let data: &str = "GET,/users,200,120
GET,/users,200,150
POST,/users,201,300
GET,/products,200,80
GET,/users,500,500
GET,/products,200,100
POST,/users,400,200
GET,/products,500,450";

    println!("{:#?}", analyze_api(data));

}


/*
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Output

(
    Vec<Request>,                    // all requests
    HashMap<&str, u32>,              // request count per endpoint
    HashMap<&str, f32>,              // average response time per endpoint
    &str,                            // busiest endpoint
    Request                          // slowest request
)


/*
Ok((
    [
        Request {
            method: "GET",
            path: "/users",
            status_code: 200,
            response_time: 120,
        },
        Request {
            method: "GET",
            path: "/users",
            status_code: 200,
            response_time: 150,
        },
        Request {
            method: "POST",
            path: "/users",
            status_code: 201,
            response_time: 300,
        },
        Request {
            method: "GET",
            path: "/products",
            status_code: 200,
            response_time: 80,
        },
        Request {
            method: "GET",
            path: "/users",
            status_code: 500,
            response_time: 500,
        },
        Request {
            method: "GET",
            path: "/products",
            status_code: 200,
            response_time: 100,
        },
        Request {
            method: "POST",
            path: "/users",
            status_code: 400,
            response_time: 200,
        },
        Request {
            method: "GET",
            path: "/products",
            status_code: 500,
            response_time: 450,
        },
    ],

    {
        "/users": 5,
        "/products": 3,
    },

    {
        "/users": 254.0,
        "/products": 210.0,
    },

    "/users",

    Request {
        method: "GET",
        path: "/users",
        status_code: 500,
        response_time: 500,
    },
))

*/



*/