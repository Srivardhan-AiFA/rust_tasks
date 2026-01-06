use crate::error::TaskError;
use crate::error::TaskError::TaskNotFound;
use crate::task::model::Task;

#[derive(Debug)]
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String) -> Task {
        let task = Task::new(self.next_id, title);
        self.next_id += 1;
        self.tasks.push(task.clone());
        task
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn complete_task(&mut self, id: u32) -> Result<&Task, TaskError> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.mark_done();
                Ok(task)
            }
            None => Err(TaskNotFound(id)),
        }
    }
}
