use std::collections::HashMap;
//use std::io::{Error};
use std::fs::{OpenOptions};
use clap::{Parser, Subcommand}; 
use prettytable::{row, Table};

pub mod error;
use error::{Result, Error};

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

    if let Err(err) = handle_actions(args) {
        println!("{}", err);
    };
}

fn handle_actions(args: Args) -> Result<()> {
    let mut todo = Todo::new().expect("Initialization of db failed");

    match args.command {
        Action::Add { item } => {
            todo.insert(item)
        },
        Action::Complete { item } => {
            todo.complete(&item)
        },
        Action::List => {
            todo.list()
        }
    }
}

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {

    fn new() -> Result<Todo> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json").map_err(|e| Error::DbInitFailed(e))?;
        
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new()
            }),
            Err(e) => panic!("An error occurred: {}", e)
        }
        
    }

    fn insert(&mut self, key: String) -> Result<()>{
        self.map.insert(key, true);
        self.save()
    }

    fn save(&self) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        println!("list updated");
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Result<()> {
        match self.map.get_mut(key) {
            Some(v) =>  { 
                *v = false; 
                self.save()
            },
            None => Err(Error::ToDoItemNotFound(key.to_string()))
        }
    }

    fn list(&self) -> Result <()> {
        let mut table = Table::new();
        table.set_titles(row!["Item", "Status"]);

        for(item, status) in &self.map {
            let status = if *status { "Pending" } else { "Done" };
            table.add_row(row![item, status]);
        }

        if table.is_empty() {
            println!("No Todo Items.");
            return Err(Error::EmptyToDos);
        } 

        println!("{}", table);
        Ok(())
    }

}