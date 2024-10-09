use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::Write;
use crate::data::{process_data::ProcessData, system_data::SystemStats};

pub fn draw_ui<W: Write>(stdout: &mut W, process_list: &[ProcessData], system_stats: &SystemStats, top_n: usize) {
    // Clear the screen and move the cursor to the top-left corner
    stdout
        .execute(terminal::Clear(ClearType::All))  // Clear the entire screen
        .unwrap()
        .execute(cursor::MoveTo(0, 0))  // Move cursor to the top-left
        .unwrap();

    stdout.flush().unwrap();  // Ensure the terminal is fully cleared

    // Print system-wide CPU and memory stats at the top
    

    // Print the header for the process table
    println!(
        "{:<6} {:<25} {:<10} {:<10}",
        "PID", "Process Name", "CPU (%)", "Memory (MB)"
    );
    println!("------------------------------------------------------------");

    // Print the top N processes in the table
    for process in process_list.iter().take(top_n) {  // Show top N processes
        println!(
            "{:<6} {:<25} {:<10.2} {:<10.2}",
            process.pid,
            process.name,
            process.cpu_usage,
            process.memory as f64 / 1024.0 / 1024.0,  // Convert memory to MB
        );
    }
    // Highlight process in red if the memory usage is above 100MB
    for process in process_list.iter().take(top_n) {
        if process.memory as f64 / 1024.0 / 1024.0 > 100.0 {
            println!("\x1b[31m{:<6} {:<25} {:<10.2} {:<10.2}\x1b[0m",
                     process.pid,
                     process.name,
                     process.cpu_usage,
                     process.memory as f64 / 1024.0 / 1024.0);
        }
    }
    println!(
        "System Stats: CPU Usage: {:.2}%, Memory Used: {:.2} MB / {:.2} MB",
        system_stats.cpu_usage,
        system_stats.memory_usage as f64 / 1024.0 / 1024.0,  // Convert to MB
        system_stats.memory_total as f64 / 1024.0 / 1024.0  // Convert to MB
    );
    println!("{}", "-".repeat(60));
    stdout.flush().unwrap();  // Ensure everything is written to the terminal
}
