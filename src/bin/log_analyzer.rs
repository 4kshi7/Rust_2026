use std::collections::HashMap;

/*
ERROR Database connection failed
ERROR Network timeout
ERROR Database connection failed
ERROR Disk full
ERROR Network timeout
ERROR Database connection failed
*/

fn count_errors(logs: &str) -> HashMap<&str, u32> {
    let mut freq: HashMap<&str, u32> = HashMap::new();

    /*
    line.split_whitespace() -> ["ERROR", "Database", "connection", "failed"]
    line.split_once(' ') -> ("ERROR", "Database connection failed")
    */

    for line in logs.lines() {
        let (severity, body) = line.split_once(" ").unwrap();

        if severity == "ERROR" {
            *freq.entry(body).or_insert(0) += 1;
        }
    }

    freq
}

fn main() {
    let log: &str = "ERROR Database connection failed
ERROR Network timeout
ERROR Database connection failed
ERROR Disk full
ERROR Network timeout
ERROR Database connection failed";

    println!("{:?}", count_errors(log));
}