mod task; //incldue task.rs as a module, make it available to call in particular functions like task::something

use clap::{Parser, Subcommand}; //use the crate for parsing CLI commands
use task::TaskManager; //call in the task manager functions from task.rs

#[derive(Parser)] //implement parsing CLI commands using clap crate
#[command(name = "Tasker", version, about = "Simple CLI Task Manager")] //use clap to add metadata to the app
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands { //enum is a type that can take any of the listed variants
    //use clap to add descriptions to commands, about for general help command, help for specific command help
    #[command(about = "Add a new task with the given description", help = "This adds a new task to the .json file, appending it to the end with an ID number one higher than the last, maximum ID number is the highest <u32>.")]
    Add {description: String},
    #[command(about = "List all tasks", help = "This lists all tasks in the .json file, their done status, and their IDs.")]
    List,
    #[command(about = "Mark a task as done by ID", help = "This marks a task as done given the ID number of it, expects <u32>.")]
    Done {id: String},
    #[command(about = "Mark a task as not done by ID", help = "This marks a task as undone given the ID number of it, expects <u32>.")]
    Undone {id: String},
    #[command(about = "Remove a task by ID", help = "This removes a task given the ID number of it, expects <u32>.")]
    Remove {id: String},
    #[command(about = "Clear all completed tasks", help = "This removes all tasks marked as done. It cannot be undone.")]
    ClearCompleted,
    #[command(about = "Remove all tasks", help = "This removes all tasks. It cannot be undone.")]
    ClearAll
}

fn main() {
    let cli = Cli::parse();
    let mut manager = TaskManager::load("tasks.json").unwrap_or_default(); //unwrap or defualt returns value if Some or Ok, or default value if not

    match cli.command { //match behaves like a switch statement, but a bit more powerful
        Commands::Add {description} => {
            manager.add_task(description);
        }
        Commands::List => {
            manager.list_tasks();
        }
        Commands::Done {id} => {
            manager.complete_task(&id);
        }
        Commands::Remove {id} => {
            manager.remove_task(&id);
        }
        Commands::ClearCompleted => {
            manager.clear_completed();
        }
        Commands::ClearAll => {
            manager.clear_all();
        }
        Commands::Undone {id} => {
            manager.mark_undone(&id);
        }
    }

    manager.save("tasks.json").expect("Failed to save tasks");
}
