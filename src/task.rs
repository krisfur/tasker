use serde::{Deserialize, Serialize};
use std::{fs, io::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task { //struct in rust is more analogous to an OOP class than to a C struct
    // define just the data held in the class here, functions belong in implementation
    id: u32,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskManager { 
    // this class has a publically available data variable that's just a vector of instances of the previous class
    pub tasks: Vec<Task>,
}


impl TaskManager { //implemenetation of the class - the place for methods!
    pub fn load(path: &str) -> Result<Self> { //Result ensures you return a TaskManager instance or error
        //&str borrows the string block without being able to mutate it, less memory used/more pointer like
        //opposite would be &mut str, but there can only be one mutable refernce at a time to avoid race conditions
        let content = fs::read_to_string(path)?; //? means this can fail and this error is to be propagated to result
        let manager: TaskManager = serde_json::from_str(&content)?;
        Ok(manager)
    }
    
    
    pub fn save(&self, path: &str) -> Result<()> {//Result<()> means succeeds or fails, no return value
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(path, content)
    }

    fn reassign_ids(&mut self) { //&mut self means the method is allowed to edit (mutate) the instance of the class
        //no Result since this method always succeeds
        for (i, task) in self.tasks.iter_mut().enumerate() {//iter_mut() iterates over each element of a vector and returns a mutable reference to each element so you can impact them
            task.id = (i as u32) + 1;
        }
    }

    pub fn add_task(&mut self, description: String) {
        let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1; //iter() returns immutable references to each vector element
    
        let task = Task {
            id: new_id,
            description,
            done: false,
        };
        
        println!("Added task [{}]: {}", task.id, task.description); //println! has "!" because it's a macro that writes and inserts its own code, lets it handle flexible formatting
        self.tasks.push(task);
    }
    

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                if task.done { "x" } else { " " },
                task.id,
                task.description
            );
        }
    }
    

    pub fn complete_task(&mut self, id_str: &str) {
        if let Ok(id) = id_str.parse::<u32>() { //if let Ok(id) is shorthand for only do stuff if id parse to u32 succeeds
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) { //Some() is a type safe way to check if value exists
                task.done = true;
                println!("Task [{}] marked as done.", id);
            } else {
                println!("Task with ID [{}] not found.", id);
            }
        } else {
            println!("Invalid task ID: {}", id_str);
        }
    }

    pub fn mark_undone(&mut self, id_str: &str) {
        if let Ok(id) = id_str.parse::<u32>() {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                if task.done {
                    task.done = false;
                    println!("Task [{}] marked as not done.", id);
                } else {
                    println!("Task [{}] is already not done.", id);
                }
            } else {
                println!("Task [{}] not found.", id);
            }
        } else {
            println!("Invalid task ID: {}", id_str);
        }
    }

    pub fn remove_task(&mut self, id_str: &str) {
        if let Ok(id) = id_str.parse::<u32>() {
            let original_len = self.tasks.len();
            self.tasks.retain(|task| task.id != id); //retain(|item| condition) keeps items that meet condition, drops the rest, all in place without making a vector copy 
    
            if self.tasks.len() < original_len {
                self.reassign_ids();
                println!("Task [{}] removed.", id);
            } else {
                println!("Task [{}] not found.", id);
            }
        } else {
            println!("Invalid task ID: {}", id_str);
        }
    }

    pub fn clear_completed(&mut self) {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| !task.done);
        let removed = original_len - self.tasks.len();
        self.reassign_ids();
        println!("Removed {} completed task(s).", removed);
    }

    pub fn clear_all(&mut self) {
        let count = self.tasks.len();
        self.tasks.clear();
        println!("Cleared all {} task(s).", count);
    }
    
}
