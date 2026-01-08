use Modular_CLI_Task_Manager::{storage::memory::TaskStore, task::model::Task};

fn main() {
    let mut task = TaskStore::new();

    print_tasks(task.list_tasks());

    let added_task = task.add_task("The Note of pram".to_string());
    println!("Task added: {:?}", added_task);

    print_tasks(task.list_tasks());

    let completed_task = task.complete_task(1);
    match completed_task {
        Ok(task) => println!("Task updated: {:?}", task),
        Err(err) => println!("Task couldn't be added cause {:?}", err),
    }

    print_tasks(task.list_tasks());

    task.add_task("The brave artist".to_string());

    print_tasks(task.list_tasks());
}

fn print_tasks(tasks: &[Task]) {
    if !tasks.is_empty() {
        println!("Total Tasks");
        for (idx, task) in tasks.iter().enumerate() {
            println!("{} {:?}", idx + 1, task);
        }
    } else {
        println!("No task to display")
    }
}
