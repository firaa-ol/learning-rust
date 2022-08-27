use std::env::args;
use std::collections::HashMap;
use std::io::{Error, Read};
use std::fs::{write, OpenOptions};
use std::str::FromStr;

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
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap())  )
            .collect();
     
        Ok(Todo {map})
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    // we are deliberately making save take ownership of the struct to
    // make sure save is the last action
    fn save(self) -> Result<(), Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        write("db.txt", content)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None
        }
    }

}