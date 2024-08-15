use std::io;
use colored::{ColoredString, Colorize};

struct TodoItem {
    id: u32,
    name: String,
    completed: bool
}


fn complete(item: &mut TodoItem) {
    item.completed = true;
}

fn display(items: &Vec<TodoItem>) {
    for item in items {
        let mut name = item.name.clone();
        
        if (item.completed) {
           name = name.strikethrough().to_string();
        }

        println!("{}. {}", item.id, name);
    }
}

fn main() {
   let mut list: Vec<TodoItem> = Vec::new();

   fn make_choice(todo_list: &mut Vec<TodoItem>) {

    println!("----------------");
    println!("----------------");
    println!("Your To do list");
    println!("----------------");
    display(&todo_list);
    println!("----------------");
    println!("1. Add a to-do item");
    println!("2. Complete a to-do item");
    println!("3. Quit");

    let mut choice = String::new();
    
    io::stdin().read_line(&mut choice).expect("Failed to read your input");

    let choice = choice.trim().parse::<u32>().expect("Invalid input");

    match choice {
        1 => {
            println!("Enter the to-do list item");

            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed to input to-do item");
            let name = name.trim().to_string();

            let id: u32 = todo_list.len() as u32 + 1;

            let item = TodoItem {
                id,
                name,
                completed: false
            };

            todo_list.push(item);
        }, 
        2 => {
            println!("Enter the id of the to-do item you want to complete");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Invalid input");
            let id = id.trim().parse::<u32>().expect("Invalid input");

            let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
            complete(item);
        }, 
        3 => {
            println!("Goodbye!");
            return;
        },
        _ => {
            println!("Invalid choice");
        }
    }

    make_choice(todo_list);
}

   make_choice(&mut list);
}
