mod process;
mod data;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, ClearType},
    ExecutableCommand,
};
use std::io::stdout;
use std::time::{Duration, Instant};
use std::{thread};
use clap::Parser;

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
    let mut sys = process::initalize_system();
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
            sys.refresh_cpu_all();  // Refresh CPU stats after the user-defined interval
            last_refresh_time = Instant::now();
        }

        sys.refresh_processes(sysinfo::ProcessesToUpdate::All);  // Refresh processes for the process list

        let process_list = process::get_process_list(&mut sys);
        let system_stats = process::get_system_stats(&mut sys);

        // Render the system stats and process list in the terminal
        ui::draw_ui(&mut stdout, &process_list, &system_stats, cli.top_n);

        // Handle user input
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('q') {
                    break; // Exit the loop if 'q' is pressed
                }
            }
        }

        // Sleep briefly before refreshing again
        thread::sleep(Duration::from_millis(200));  // Sleep for 200ms
    }

    // Clean up the terminal on exit
    terminal::disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();  // Leave alternate screen
}
