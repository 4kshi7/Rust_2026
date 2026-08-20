use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Server {
    name: String,
    server_type: String,
    cpu_usage: u32,
    response_time: u32,
    status: String,
}

fn find_highest_cpu(servers: &[Server]) -> Option<&Server> {
    if servers.is_empty() {
        return None;
    }

    let mut highest: &Server = servers.first()?;

    for server in servers {
        if server.cpu_usage > highest.cpu_usage {
            highest = server;
        }
    }

    Some(highest)
}

fn main(){
    let servers = vec![
    Server {
        name: "server-1".to_string(),
        server_type: "web".to_string(),
        cpu_usage: 85,
        response_time: 120,
        status: "healthy".to_string(),
    },
    Server {
        name: "server-2".to_string(),
        server_type: "web".to_string(),
        cpu_usage: 92,
        response_time: 180,
        status: "healthy".to_string(),
    },
    // ...
];
    println!("{:#?}", find_highest_cpu(&servers));

}
