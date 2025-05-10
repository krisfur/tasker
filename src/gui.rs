use eframe::egui;
use crate::task::TaskManager;

pub struct TaskApp {
    manager: TaskManager,
    new_task_desc: String,
}

impl Default for TaskApp {
    fn default() -> Self {
        Self {
            manager: TaskManager::load("tasks.json").unwrap_or_default(),
            new_task_desc: String::new(),
        }
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Manager");

            // Input to add new task
            ui.horizontal(|ui| {
                let input_id = ui.make_persistent_id("task_input");

                let text_response = ui.add(
                    egui::TextEdit::singleline(&mut self.new_task_desc)
                        .hint_text("Enter a new task")
                        .id(input_id),
                );

                // Pressing Enter in the text field
                let should_add = text_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if (should_add) && !self.new_task_desc.trim().is_empty() {
                    self.manager.add_task(self.new_task_desc.trim().to_string());
                    self.new_task_desc.clear();
                    self.manager.save("tasks.json").ok();

                    // Re-focus the input box
                    ui.memory_mut(|mem| mem.request_focus(input_id));
                }
            });

            ui.separator();

            let mut changed = false;
            let mut to_delete: Option<u32> = None;

            for task in &mut self.manager.tasks {
                ui.horizontal(|ui| {
                    //Checkbox to toggle done
                    let response = ui.checkbox(&mut task.done, format!("{}: {}", task.id, task.description));
                    if response.changed() {
                        changed = true;  //checkbox toggled
                    }

                    //"X" button to delete
                    if ui.button("🗑").clicked() {
                        to_delete = Some(task.id);
                    }
                });
            }

            // Remove the task (outside of the UI closure)
            if let Some(id) = to_delete {
                self.manager.remove_task(&id.to_string());
                changed = true;
            }

            // Save *after* the loop, outside of the UI closures
            if changed {
                self.manager.save("tasks.json").ok();
            }

            if ui.button("Clear Completed").clicked() {
                self.manager.clear_completed();
            }
        });
    }
}
