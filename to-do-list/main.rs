use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut todo_list = load_todo_list().unwrap_or_else(|| Vec::new());

    loop {
        println!("\nEnter a command: (add/remove/show/save/quit)");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim().to_lowercase();

        match command.as_str() {
            "add" => {
                println!("Enter task:");
                let mut task_input = String::new();
                std::io::stdin()
                    .read_line(&mut task_input)
                    .expect("Failed to read line");
                let task = task_input.trim().to_string();
                todo_list.push(task);
                println!("Task added successfully");
            }
            "remove" => {
                println!("Enter index to remove:");
                let mut index_input = String::new();
                std::io::stdin()
                    .read_line(&mut index_input)
                    .expect("Failed to read line");
                let index: usize = match index_input.trim().parse() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Invalid index");
                        continue;
                    }
                };
                if index >= todo_list.len() {
                    println!("Invalid index");
                } else {
                    todo_list.remove(index);
                    println!("Task removed successfully");
                }
            }
            "show" => {
                if todo_list.is_empty() {
                    println!("No tasks");
                } else {
                    for (index, task) in todo_list.iter().enumerate() {
                        println!("{}: {}", index, task);
                    }
                }
            }
            "save" => {
                save_todo_list(&todo_list).unwrap();
                println!("Todo list saved successfully");
            }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

fn load_todo_list() -> Option<Vec<String>> {
    let file = File::open("todo.txt").ok()?;
    let reader = BufReader::new(file);
    let mut todo_list = Vec::new();
    for line in reader.lines() {
        if let Ok(task) = line {
            todo_list.push(task);
        }
    }
    Some(todo_list)
}

fn save_todo_list(todo_list: &Vec<String>) -> std::io::Result<()> {
    let mut file = File::create("todo.txt")?;
    for task in todo_list {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}
