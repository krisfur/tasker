#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg(windows)]

mod task; //incldue task.rs as a module, make it available to call in particular functions like task::something
mod gui;

use clap::{Parser, Subcommand}; //use the crate for parsing CLI commands

#[derive(Parser)] //implement parsing CLI commands using clap crate
#[command(name = "Tasker", version, about = "Simple CLI Task Manager")] //use clap to add metadata to the app
struct Cli {
    // Launch the GUI version
    #[arg(long)]
    gui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands { //enum is a type that can take any of the listed variants
    //use clap to add descriptions to commands, about for general help command, help for specific command help
    #[command(about = "Add a new task with the given description", long_about = "This adds a new task to the .json file, appending it to the end with an ID number one higher than the last, maximum ID number is the highest <u32>.")]
    Add {description: String},
    #[command(about = "List all tasks", long_about = "This lists all tasks in the .json file, their done status, and their IDs.")]
    List,
    #[command(about = "Mark a task as done by ID", long_about = "This marks a task as done given the ID number of it, expects <u32>.")]
    Done {id: String},
    #[command(about = "Mark a task as not done by ID", long_about = "This marks a task as undone given the ID number of it, expects <u32>.")]
    Undone {id: String},
    #[command(about = "Remove a task by ID", long_about = "This removes a task given the ID number of it, expects <u32>.")]
    Remove {id: String},
    #[command(about = "Clear all completed tasks", long_about = "This removes all tasks marked as done. It cannot be undone.")]
    ClearCompleted,
    #[command(about = "Remove all tasks", long_about = "This removes all tasks. It cannot be undone.")]
    ClearAll
}

fn main() -> Result<(), eframe::Error> {
    let cli = Cli::parse();

    // Default to GUI if no command is provided and --gui isn't used explicitly
    let should_run_gui = cli.gui || cli.command.is_none();

    if should_run_gui {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Task Manager",
            options,
            Box::new(|_cc| Box::new(gui::TaskApp::default())),
        )?;
        return Ok(());
    }

    // CLI mode
    if let Some(command) = cli.command {
        let mut manager = task::TaskManager::load("tasks.json").unwrap_or_default();

        match command {
            Commands::Add { description } => manager.add_task(description),
            Commands::List => manager.list_tasks(),
            Commands::Done { id } => manager.complete_task(&id),
            Commands::Undone { id } => manager.mark_undone(&id),
            Commands::Remove { id } => manager.remove_task(&id),
            Commands::ClearCompleted => manager.clear_completed(),
            Commands::ClearAll => manager.clear_all(),
        }

        manager.save("tasks.json").ok();
    }

    Ok(())
}