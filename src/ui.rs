use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
    QueueableCommand
};
use std::io::Write;
use crate::data::{process_data::ProcessData, system_data::SystemStats};

pub fn draw_ui<W: Write>(stdout: &mut W, process_list: &[ProcessData], system_stats: &SystemStats, top_n: usize) {
    // Clear the screen and move the cursor to the top-left corner
    stdout
        .execute(terminal::Clear(ClearType::Purge))  // Clear the entire screen
        .unwrap()
        .execute(cursor::MoveTo(0, 0))  // Move cursor to the top-left
        .unwrap();

    stdout.flush().unwrap();  // Ensure the terminal is fully cleared

    // Print system-wide CPU and memory stats at the top
    // Format it like [xxxxx], where x is a # character, and the number of x's is the CPU usage over 5
    println!(
        "System-wide CPU usage: [{:<5}] {:.2}%, Memory usage: {:.2} MB / {:.2} MB, press 'q' to quit",
        "#".repeat((system_stats.cpu_usage / 5.0) as usize),
        system_stats.cpu_usage,
        system_stats.memory_usage as f64 / 1024.0 / 1024.0,  // Convert memory to MB
        system_stats.memory_total as f64 / 1024.0 / 1024.0,  // Convert memory to MB
    );

    // Print a separator for readability
    println!("------------------------------------------------------------");

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
        // Highlight processes using more than 1GB of memory
        if process.memory >= 1024 * 1024 * 1024 {
            stdout.queue(crossterm::style::SetForegroundColor(crossterm::style::Color::Red)).unwrap();
        }
        if process.cpu_usage >= 50.0 {
            stdout.queue(crossterm::style::SetForegroundColor(crossterm::style::Color::Yellow)).unwrap();
        }
        stdout.flush().unwrap();  // Flush the output to the terminal
    }

    // Flush the output to the terminal
    stdout.flush().unwrap();
}
