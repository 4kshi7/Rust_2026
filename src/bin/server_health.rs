use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Server {
    name: String,
    server_type: String,
    cpu_usage: u32,
    response_time: u32,
    status: String,
}

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

fn analyze_servers(data: &str) -> Result<(Vec<Server>, HashMap<String, u32>, HashMap<String, u32>, Server, Server, f32), String> {

    let mut server_vec:Vec<Server> = Vec::new();
    for line in data.lines(){
        let mut values = line.split(',');

        let name = parse_string(values.next(),"name")?;
        let server_type = parse_string(values.next(),"server_type")?;
        let cpu_usage = parse_u32(values.next(),"cpu_usage")?;
        let response_time = parse_u32(values.next(),"response_time")?;
        let status = parse_string(values.next(),"status")?;

        let server_data = Server{
            name,
            server_type,
            cpu_usage,
            response_time,
            status
        };

        server_vec.push(server_data);
    }

    //type and health count
    let mut count_map:HashMap<String, u32> = HashMap::new();
    let mut health_map:HashMap<String, u32> = HashMap::new();
    
    let mut highest_cpu_usage:u32 = 0;
    let mut highest_cpu_usage_index:usize = 0;

    let mut highest_response_time:u32 = 0;
    let mut highest_response_time_index:usize = 0;

    let mut total_cpu_usage : f32 = 0.0;
    let total_servers: f32 = server_vec.len() as f32;

    for (i, data) in server_vec.iter().enumerate(){
        *count_map.entry(data.server_type.clone()).or_insert(0) +=1;
        *health_map.entry(data.status.clone()).or_insert(0) +=1;
        
        if data.cpu_usage > highest_cpu_usage {
            highest_cpu_usage = data.cpu_usage;
            highest_cpu_usage_index = i;
        }

        if data.response_time > highest_response_time {
            highest_response_time = data.response_time;
            highest_response_time_index = i;
        }

        total_cpu_usage += data.cpu_usage as f32;
    }
    
    let avg_cpu:f32 = total_cpu_usage/total_servers;

    let high_cpu_vec = server_vec[highest_cpu_usage_index].clone();
    let high_response_vec = server_vec[highest_response_time_index].clone();


    Ok((server_vec, count_map, health_map, high_cpu_vec, high_response_vec, avg_cpu))
}

fn main(){
    let data: &str = "server-1,web,85,120,healthy
server-2,web,92,180,healthy
server-3,db,78,250,warning
server-4,web,97,310,critical
server-5,db,65,140,healthy
server-6,cache,88,90,healthy
server-7,db,91,400,critical
server-8,cache,72,110,warning";
    println!("{:#?}", analyze_servers(data));

}


/*

Vec<Server>

HashMap<String, u32>  // type counts

HashMap<String, u32>  // status counts

Server                 // highest CPU

Server                 // slowest response

f32                    // average CPU

Ok((
    [
        Server {
            name: "server-1",
            server_type: "web",
            cpu_usage: 85,
            response_time: 120,
            status: "healthy",
        },
        Server {
            name: "server-2",
            server_type: "web",
            cpu_usage: 92,
            response_time: 180,
            status: "healthy",
        },
        Server {
            name: "server-3",
            server_type: "db",
            cpu_usage: 78,
            response_time: 250,
            status: "warning",
        },
        Server {
            name: "server-4",
            server_type: "web",
            cpu_usage: 97,
            response_time: 310,
            status: "critical",
        },
        Server {
            name: "server-5",
            server_type: "db",
            cpu_usage: 65,
            response_time: 140,
            status: "healthy",
        },
        Server {
            name: "server-6",
            server_type: "cache",
            cpu_usage: 88,
            response_time: 90,
            status: "healthy",
        },
        Server {
            name: "server-7",
            server_type: "db",
            cpu_usage: 91,
            response_time: 400,
            status: "critical",
        },
        Server {
            name: "server-8",
            server_type: "cache",
            cpu_usage: 72,
            response_time: 110,
            status: "warning",
        },
    ],

    {
        "web": 3,
        "db": 3,
        "cache": 2,
    },

    {
        "healthy": 4,
        "warning": 2,
        "critical": 2,
    },

    Server {
        name: "server-4",
        server_type: "web",
        cpu_usage: 97,
        response_time: 310,
        status: "critical",
    },

    Server {
        name: "server-7",
        server_type: "db",
        cpu_usage: 91,
        response_time: 400,
        status: "critical",
    },

    83.5,
))

*/
