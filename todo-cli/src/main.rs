use std::collections::HashMap;
use std::io::{Error};
use std::fs::{OpenOptions};
use clap::{Parser, Subcommand}; 

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Action
}

#[derive(Subcommand)]
enum Action {
    /// Add a new todo item
    Add {
        /// name of the todo item to add
        item: String
    },
    /// Complete a given todo item
    Complete {
        /// name of the todo item to complete
        item: String
    },
    /// List todo items
    List 
}

fn main() {
    let args = Args::parse();

    let mut todo = Todo::new().expect("Initialization of db failed");

    match args.command {
        Action::Add { item } => {
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why)
            }
        },
        Action::Complete { item } => {
            match todo.complete(&item) {
                None => println!("'{}' is not present in the list", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why)
                }
            }
        },
        Action::List => {
            println!("Item\t\t\tStatus");
            for(item, status) in &todo.map {
                let status = if *status { "Pending" } else { "Done" };
                println!("{item}\t\t{status}");
            }
        }
    }
}

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {

    fn new() -> Result<Todo, Error> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new()
            }),
            Err(e) => panic!("An error occurred: {}", e)
        }
        
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    // we are deliberately making save take ownership of the struct to
    // make sure save is the last action
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None
        }
    }

}