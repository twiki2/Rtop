use std::process::Command;
use std::str;

pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub status: String,
}

impl ProcessInfo {
    pub fn new(pid: i32, name: String, status: String) -> ProcessInfo {
        ProcessInfo { pid, name, status }
    }
}

pub fn get_processes() -> Vec<ProcessInfo> {
    let output = Command::new("ps")
        .arg("aux")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to parse output");
    parse_processes(output_str)
}

fn parse_processes(output: &str) -> Vec<ProcessInfo> {
    let mut processes = Vec::new();
    let lines: Vec<&str> = output.lines().skip(1).collect(); 

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            let pid = parts[1].parse::<i32>().unwrap_or(-1);
            let name = parts[10].to_string();
            let status = parts[7].to_string();

            let process_info = ProcessInfo::new(pid, name, status);
            processes.push(process_info);
        }
    }

    processes
}

pub fn display_processes(processes: &[ProcessInfo]) {
    println!("PID\tName\tStatus");
    for process in processes {
        println!("{}\t{}\t{}", process.pid, process.name, process.status);
    }
}
