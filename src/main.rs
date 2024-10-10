mod process;
mod ui;
mod data;

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{io::stdout, thread, time::Duration};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

#[derive(Parser)]
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
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    let mut stdout = stdout();

    // Set up terminal UI
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();

    // Main event loop
    loop {
        // Allow CPU usage time to calculate properly
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        // Refresh processes with specific data (CPU usage)
        system.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            sysinfo::ProcessRefreshKind::new().with_cpu(),
        );

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        system.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            sysinfo::ProcessRefreshKind::new().with_cpu(),
        );

        // Get process list and system stats
        let process_list = process::get_process_list(&mut system);
        let system_stats = process::get_system_stats(&mut system);

        // Render the system stats and process list
        ui::draw_ui(&mut stdout, &process_list, &system_stats, cli.top_n);

        // Handle user input (quit with 'q')
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Sleep briefly before the next refresh
        thread::sleep(Duration::from_secs(cli.interval));
    }

    // Clean up terminal on exit
    terminal::disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
}
