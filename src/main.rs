use std::net::TcpStream;
use std::time::{SystemTime, Duration};
use std::thread;
use std::io::prelude::*;
use chrono::prelude::*;

fn main() -> std::io::Result<()> {
    loop {
        // Get the current system time
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();

        // Format the date and time
        let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        // Try to connect and send data
        // change addr to appropriate IP address
        match TcpStream::connect("000.000.00.00:7878") {
            Ok(mut stream) => {
                if let Err(e) = stream.write_all(datetime_str.as_bytes()) {
                    eprintln!("Failed to send data: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to connect: {}", e),
        }

        // Sleep for 5 minutes (300 seconds)
        thread::sleep(Duration::from_secs(300));
    }
}