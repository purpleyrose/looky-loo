mod process;
mod data;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;
use std::time::{Duration, Instant};
use std::thread;
use clap::Parser;
use sysinfo::System;

/// A process monitoring tool similar to top.
#[derive(Parser)]
#[command(name = "process_monitor")]
#[command(about = "A simple terminal-based process monitor", long_about = None)]
struct Cli {
    /// Number of top processes to show
    #[arg(short, long, default_value_t = 10)]
    top_n: usize,

    /// Refresh interval in seconds
    #[arg(short, long, default_value_t = 1)]
    interval: u64,
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize system and terminal
    let mut sys = System::new_all(); // Initialize System with all information
    let mut stdout = stdout();

    // Set up the terminal UI (enter alternate screen)
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();  // Enter alternate screen mode

    // Track the time of the last refresh for CPU calculation
    let mut last_refresh_time = Instant::now();

    // Main event loop
    loop {
        let elapsed = last_refresh_time.elapsed();
        if elapsed >= Duration::from_secs(cli.interval) {
            sys.refresh_all();  // Refresh all system information (CPU, memory, processes)
            
            last_refresh_time = Instant::now();
        }

        // Introduce a delay between refreshes to allow CPU usage to accumulate
        thread::sleep(Duration::from_secs(1));  // 1 second delay for CPU usage calculation

        // Retrieve updated system stats and process list
        let process_list = process::get_process_list(&mut sys);
        let system_stats = process::get_system_stats(&mut sys);

        // Render the system stats and process list in the terminal
        ui::draw_ui(&mut stdout, &process_list, &system_stats, cli.top_n);

        // Handle user input (to quit)
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('q') {
                    // clear the screen
                    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

                    break; // Exit the loop if 'q' is pressed
                    
                }
            }
        }
    }

    // Clean up the terminal on exit
    terminal::disable_raw_mode().unwrap();
    // clear the screen
    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge)).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();  // Leave alternate screen
}
