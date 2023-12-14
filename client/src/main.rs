use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Polling server for command
        thread::sleep(Duration::from_secs(10));
        // If server indicates to run the script:
        run_powershell_script();
    }
}

fn run_powershell_script() {
    let output = Command::new("powershell")
        .arg("./scripts/script1.ps1")  
        .output()
        .expect("Failed to execute script");

    println!("Script output: {:?}", output);
}
