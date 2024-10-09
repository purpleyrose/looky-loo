# Process Manager App

## Description

This is a simple terminal based application that allows users to manage their processes. The application allows users to view all running processes and kill any process of their choice.

## Features

- View all running processes
- Kill any process of your choice
- Exit the application
- View the application's help menu
- View the application's version
- Sort processes by PID, Name, or Memory Usage, either in the app or as a command line argument
  
## Tech flow

- Gather all running processes, using the `sysinfo` crate
- Use `crossterm` to display the processes in a table format
- Use `crossterm` to display the help menu
- Use `clap` to parse command line arguments
- Use `sysinfo` to kill a process
- Use `async-std` to run the application asynchronously, allowing user input and process killing to run concurrently

## Checklist

- [] Gather all running processes, ideally in minimal time
- [] Display the processes in a table format
