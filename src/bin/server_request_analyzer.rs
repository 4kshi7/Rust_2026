#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    status_code: u32,
    response_time: u32
}

fn analyze_requests(data: &str) -> Result<(Vec<Request>, u32, f32, u32, u32, Request), String>{

    //all requests
    let mut request_vector:Vec<Request> = Vec::new();

    for line in data.lines(){
        let mut values = line.split(',');
 
        let method = match values.next(){
            Some(val) => val,
            None => return Err("Invalid formatting for method".to_string()),
        };

        let path = match values.next(){
            Some(val) => val,
            None => return Err("Invalid formatting for path".to_string()),
        };

        let status_code_str = match values.next(){
            Some(val) => val,
            None => return Err("Invalid formatting for status_code_str".to_string()),
        };

        let status_code:u32 = match status_code_str.parse(){
            Ok(val) => val,
            Err(_) => return Err("Invalid formatting for status_code".to_string()),
        };

        let response_time_str = match values.next(){
            Some(val) => val,
            None => return Err("Invalid formatting for response_time_str".to_string()),
        };

        let response_time:u32 = match response_time_str.parse(){
            Ok(val) => val,
            Err(_) => return Err("Invalid formatting for status_time".to_string()),
        };
        
        let web_data = Request{
            method: method.to_string(),
            path: path.to_string(),
            status_code,
            response_time,
        };

        request_vector.push(web_data);

    }


    //total number of request
    let mut total_request:u32 = 0;
    let mut total_time:u32 = 0;
    let mut success_count:u32 = 0;
    let mut failed_count:u32 = 0;
    let mut max_time:u32 = 0;
    let mut max_time_index:usize = 0;

    for (i, data) in request_vector.iter().enumerate(){
        total_request += 1;
        total_time += data.response_time;

         //number of successful requests
        if data.status_code < 400 {
            success_count += 1;
        }

        //number of failed requests
        if data.status_code > 400 {
            failed_count += 1;
        }

        //slowest request
        let value:u32 = data.response_time;
        if value > max_time {
            max_time = value;
            max_time_index = i;
            
        }
        
    }
    
    //average response time
    let avg_time:f32 = total_time as f32 / total_request as f32;

    //slowest request continues

    let slowest_req : Request = request_vector[max_time_index].clone();


    Ok((request_vector, total_request, avg_time, success_count, failed_count, slowest_req))

}

fn main(){
    let data: &str = "GET,/home,200,120
POST,/login,200,250
GET,/products,200,180
GET,/home,200,90
POST,/login,401,110
GET,/products,500,400
GET,/home,200,100
POST,/checkout,500,350";


    println!("{:#?}", analyze_requests(data));
}

/* 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Output

Ok((
    [
        Request { method: "GET", path: "/home", status_code: 200, response_time: 120 },
        Request { method: "POST", path: "/login", status_code: 200, response_time: 250 },
        Request { method: "GET", path: "/products", status_code: 200, response_time: 180 },
        Request { method: "GET", path: "/home", status_code: 200, response_time: 90 },
        Request { method: "POST", path: "/login", status_code: 401, response_time: 110 },
        Request { method: "GET", path: "/products", status_code: 500, response_time: 400 },
        Request { method: "GET", path: "/home", status_code: 200, response_time: 100 },
        Request { method: "POST", path: "/checkout", status_code: 500, response_time: 350 }
    ],
    8,
    200.0,
    5,
    3,
    Request { method: "GET", path: "/products", status_code: 500, response_time: 400 }
))

(
    Vec<Request>,       // all requests
    8,                  // total requests
    200.0,              // average response time
    5,                  // successful
    3,                  // failed
    slowest Request
)


*/