/*
Leonardo L. Rapanan
Class: SPT323
Assignment: Final Project */

use std::io;
// Function Todo Itam set as strings for to do list.
struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}
// Function completed Item is set true once completed using bool.
fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}
// Function display Item(s) for all items completed or need to be completed.
fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}

// function name for user input.
fn user (s: &str){
    println!("{}", s);
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    // Program tells user to enter their name.
   println!("Enter your name: ");
   let mut name = String::new();
     std::io::stdin().read_line(&mut name).expect("Failed to read line");
   print!("Welcome! ");
   user(&name);

    loop {
        // Program asks user what they like to do. Printing their name.
        print!("What would you like to do? ");
        user(&name);
        // options for user.
        println!("1. Add a to-do item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do items");
        println!("4. Quit\n");
        // User will input choice as a number.
        println!("Choice: ");
        // User inputs the number.
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().expect("Invalid input");
        // Using match choice for user input.
        match choice {
            // When user inputs number 1 on the option list.
            1 => {
                // Program tells user to enter a to-do Item as a string.
                println!("\nEnter the name of the to-do item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;
                /*To do item list inputed and saved. Will remain incomplete
                Unitl user chooses option 2 to mark task as comlete. */
                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            },
            // When user chooses option 2.
            2 => {
                // Program tells user which list of items they want to complete.
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            },
            // When user inputs option 3.
            3 => {
                // Program displays to-do list items.
                display_items(&todo_list);
            },
            // user inputs option 4.
            4 => {
                //Program quits when user inputs 4.
                println!("Goodbye!");
                return;
            },
            // When user inputs other number than 1 - 4.
            _ => {
                println!("Invalid choice");
            },
        }
    }
}
