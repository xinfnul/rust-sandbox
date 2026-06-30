use colored::*;
use std::fs::{self, File};
use std::io::{self, Write};

const FILE_PATH: &str = "todos.txt";

#[derive(Debug)]
struct Todo {
    id: u32,
    name: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            completed: false,
        }
    }

    fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

struct TodoApp {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoApp {
    fn new() -> Self {
        let mut app = Self {
            todos: Vec::new(),
            next_id: 1,
        };

        app.load();

        app
    }

    fn load(&mut self) {
        let content = match fs::read_to_string(FILE_PATH) {
            Ok(c) => c,
            Err(_) => return,
        };

        let mut max_id = 0;

        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() != 3 {
                continue;
            }

            let id = match parts[0].parse::<u32>() {
                Ok(id) => id,
                Err(_) => continue,
            };

            let completed = match parts[2].parse::<bool>() {
                Ok(v) => v,
                Err(_) => false,
            };

            self.todos.push(Todo {
                id,
                name: parts[1].to_string(),
                completed,
            });

            if id > max_id {
                max_id = id;
            }
        }

        self.next_id = max_id + 1;
    }

    fn save(&self) -> io::Result<()> {
        let mut file = File::create(FILE_PATH)?;

        for todo in &self.todos {
            writeln!(file, "{}|{}|{}", todo.id, todo.name, todo.completed)?;
        }

        Ok(())
    }

    fn add(&mut self, name: String) {
        let todo = Todo::new(self.next_id, name);
        self.todos.push(todo);
        self.next_id += 1;
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!();
            println!("{}", "Todo list is empty.".red().bold());
            println!();
            return;
        }

        println!();
        println!(
            "{}",
            "================================================="
                .blue()
                .bold()
        );
        println!(
            "{}",
            "                    TODO LIST                     "
                .bright_white()
                .bold()
        );
        println!(
            "{}",
            "================================================="
                .blue()
                .bold()
        );

        for todo in &self.todos {
            let status = if todo.completed {
                "[DONE]".green().bold()
            } else {
                "[PENDING]".yellow().bold()
            };

            println!(
                "{}",
                "-------------------------------------------------".bright_black()
            );
            println!("{} {}", "ID     :".cyan().bold(), todo.id);
            println!("{} {}", "Task   :".cyan().bold(), todo.name);
            println!("{} {}", "Status :".cyan().bold(), status);
        }

        println!(
            "{}",
            "-------------------------------------------------".bright_black()
        );
        println!();
    }

    fn toggle_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.toggle();
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> bool {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            true
        } else {
            false
        }
    }

    fn cleanup(&mut self) -> io::Result<()> {
        self.todos.clear();
        self.next_id = 1;

        if fs::metadata(FILE_PATH).is_ok() {
            fs::remove_file(FILE_PATH)?;
        }

        Ok(())
    }
}

fn main() {
    println!();
    println!(
        "{}",
        "================================================="
            .blue()
            .bold()
    );
    println!(
        "{}",
        "                  TODO MANAGER                    "
            .green()
            .bold()
    );
    println!(
        "{}",
        "================================================="
            .blue()
            .bold()
    );
    println!();

    let mut app = TodoApp::new();

    loop {
        println!("{}", "Menu".yellow().bold());
        println!(
            "{}",
            "-------------------------------------------------".bright_black()
        );

        println!("{} Add Todo", "1.".green().bold());
        println!("{} List Todos", "2.".cyan().bold());
        println!("{} Remove Todo", "3.".red().bold());
        println!("{} Toggle Completed", "4.".magenta().bold());
        println!("{} Exit", "5.".yellow().bold());
        println!("{} Cleanup", "6.".bright_red().bold());

        println!(
            "{}",
            "-------------------------------------------------".bright_black()
        );

        let choice = match read_input("Enter your choice: ") {
            Ok(input) => input,
            Err(e) => {
                eprintln!("{} {}", "Input error:".red().bold(), e);
                continue;
            }
        };

        let choice = match choice.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("{}", "Please enter a valid choice.".red().bold());
                println!();
                continue;
            }
        };

        match choice {
            1 => {
                let name = match read_input("Enter todo name: ") {
                    Ok(name) if !name.is_empty() => name,
                    _ => {
                        println!();
                        println!("{}", "Todo name cannot be empty.".red().bold());
                        println!();
                        continue;
                    }
                };

                app.add(name);

                if let Err(e) = app.save() {
                    eprintln!("Failed to save: {}", e);
                }

                println!();
                println!("{}", "Todo added successfully.".green().bold());
                println!();
            }

            2 => app.list(),

            3 => {
                let Some(id) = read_u32("Enter todo id: ") else {
                    println!();
                    println!("{}", "Enter a valid id.".red().bold());
                    println!();
                    continue;
                };

                if app.delete(id) {
                    if let Err(e) = app.save() {
                        eprintln!("Failed to save: {}", e);
                    }

                    println!();
                    println!("{}", "Todo deleted successfully.".green().bold());
                    println!();
                } else {
                    println!();
                    println!("{}", "No todo found with that id.".red().bold());
                    println!();
                }
            }

            4 => {
                let Some(id) = read_u32("Enter todo id: ") else {
                    println!();
                    println!("{}", "Enter a valid id.".red().bold());
                    println!();
                    continue;
                };

                if app.toggle_completed(id) {
                    if let Err(e) = app.save() {
                        eprintln!("Failed to save: {}", e);
                    }

                    println!();
                    println!("{}", "Todo status updated.".green().bold());
                    println!();
                } else {
                    println!();
                    println!("{}", "No todo found with that id.".red().bold());
                    println!();
                }
            }

            5 => {
                println!();
                println!("{}", "Exiting Todo Manager...".yellow().bold());
                println!();
                break;
            }

            6 => {
                println!();
                if let Err(_) = app.cleanup() {
                    eprintln!("{}", "Error deleting file".red());
                }
            }

            _ => {
                println!();
                println!("{}", "Please enter a valid choice.".red().bold());
                println!();
            }
        }
    }
}

fn read_input(prompt: &str) -> io::Result<String> {
    print!("{}", format!("> {}", prompt).bright_blue().bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn read_u32(prompt: &str) -> Option<u32> {
    match read_input(prompt) {
        Ok(input) => input.parse().ok(),
        Err(_) => None,
    }
}
