use std::env::args;
use std::collections::HashMap;
use std::io::{Error};
use std::fs::{OpenOptions};

fn main() {
    let action = args().nth(1).expect("please specify an action");
    let item = args().nth(2).expect("please specify an item");

    let mut todo = Todo::new().expect("Initialization of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why)
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why)
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