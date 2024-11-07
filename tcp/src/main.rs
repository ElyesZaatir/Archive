use sysinfo::System;
use std::process::{Command, exit};
use std::thread::{self, sleep};
use std::time::Duration;
use std::io::{BufRead, BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

fn main() {
    // Start the TCP server in a separate thread
    let _server_thread = thread::spawn(|| {
        start_tcp_server("127.0.0.1:7878"); // Change to your desired server address and port
    });

    let process_name = "";
    let desired_ip = ""; // Change to the IP address you want to monitor
    let check_interval = Duration::from_secs(25); // Check every 25 seconds

    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Check if the specified process is running
        if let Some(process) = sys.processes().values().find(|p| p.name() == process_name) {
            println!("Process {} is running (PID: {}).", process_name, process.pid());

            // Check for active connections to the desired IP
            if !has_active_connection(process.pid().as_u32(), desired_ip) {
                println!("No active connection to {}. Killing and restarting process...", desired_ip);
                // Kill the process explicitly
                Command::new("taskkill")
                    .args(&["/F", "/PID", &process.pid().to_string()])
                    .output()
                    .expect("Failed to kill process");

                // Restart the process
                match Command::new(process_name).spawn() {
                    Ok(_) => {
                        println!("Successfully restarted the process.");
                    },
                    Err(e) => {
                        eprintln!("Failed to restart process: {:?}", e);
                        if e.kind() == ErrorKind::NotFound {
                            eprint!("Please put the program in the same directory as Process.exe\n");
                        }
                        eprint!("Closing in 10 seconds...");
                        sleep(Duration::from_secs(10));
                        exit(1);
                    }
                }
            } else {
                println!("Active connection to {} found.", desired_ip);
            }
        } else {
            println!("Process {} not found. Attempting to start...", process_name);
            match Command::new(process_name).spawn() {
                Ok(_) => {
                    println!("Successfully started the process.");
                },
                Err(e) => {
                    eprintln!("Failed to start process: {:?}", e);
                    if e.kind() == ErrorKind::NotFound {
                        eprint!("Please put the program in the same directory as Process.exe\n");
                    }
                    eprint!("Closing in 10 seconds...");
                    sleep(Duration::from_secs(10));
                    exit(1);
                }
            }
        }

        // Sleep before checking again
        thread::sleep(check_interval);
    }
}

// Function to start a TCP server
fn start_tcp_server<A: ToSocketAddrs>(addr: A) {
    let listener = TcpListener::bind(addr).expect("Could not bind to address");
    println!("TCP server listening");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream);
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
            }
        }
    }
}

// Function to handle incoming TCP connections
fn handle_connection(stream: TcpStream) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(message) => println!("Received: {}", message),
            Err(e) => eprintln!("Error reading from stream: {:?}", e),
        }
    }
}

// Function to check for active connections to the specified IP for a process
fn has_active_connection(pid: u32, desired_ip: &str) -> bool {
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output()
        .expect("Failed to execute netstat command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        // Check if line contains both the desired IP and the PID
        if line.contains(desired_ip) && line.contains(&pid.to_string()) {
            return true; // Found a connection to the desired IP
        }
    }
    false // No connection found
}
