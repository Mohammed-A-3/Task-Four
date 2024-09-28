use chrono::Local;

fn main() {
    // Get the current local time
    let current_time = Local::now();
    
    // Print the message
    println!("Hello Mohammed A, right now the time is {}", current_time.format("%Y-%m-%d %H:%M:%S"));
}

