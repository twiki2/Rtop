use std::process::Command;
use std::time::Duration;
use std::thread;
mod system_info;

fn main() {
    loop {
        let processes = system_info::get_processes();
        system_info::display_processes(&processes);
        thread::sleep(Duration::from_secs(1)); 
    }
}
