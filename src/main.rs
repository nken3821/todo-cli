use clap::Parser;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TodoList {
    // True = to do / False = done
    items : HashMap<String, bool>,
}

impl TodoList {
    fn new() -> TodoList {
        let items : HashMap<String, bool> = HashMap::new();
        TodoList {items}
    }

    fn add(&mut self, key: String ) {
        if let Entry::Vacant(entry ) = self.items.entry(key) {
            entry.insert(true);
        }
    }

    fn remove(&mut self, key: String) -> Result<String, String>{
        match self.items.remove(&key) {
            Some(_) => Ok(key),
            None => Err(key)
        }        
    }

    fn mark(&mut self, key: String, value: bool) -> Result<String, String>{
        let x = self.items.get_mut(&key).ok_or(&key)?;
        *x = value;
        Ok(key)
    }

    fn list(& self) -> (impl Iterator<Item = &String>, impl Iterator<Item = &String>) {
        (
            self.items.iter().filter(|x| *x.1 == true).map(|x| x.0),
            self.items.iter().filter(|x| *x.1 == false).map(|x| x.0),
        )
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let _ = std::fs::write("todo.json", data);
        Ok(())
    }

    fn load() -> Result<TodoList, Box<dyn std::error::Error>> {
        match std::fs::read_to_string("todo.json") {
            Ok(data) => Ok(serde_json::from_str(&data)?),
            Err(_) => Ok(TodoList::new()),
        }
    }
}

#[derive(Parser)]
struct Cli {
    command: String,
    key: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    let mut todo = TodoList::load()?;


    let result = match args.command.as_str() {
        "add" => match args.key {
            Some(key) => {
                todo.add(key);
                todo.save()?;
                Ok(())
            }
            None => Err("Key cannot be empty!".to_string()),
        },

        "remove" => match args.key {
            Some(key) => {
                todo.remove(key).map_err(|e| format!("Invalid key {}", e))?;
                todo.save()?;
                Ok(())
            }
            None => Err("Key cannot be empty!".to_string()),
        },

        "mark-done" => match args.key {
            Some(key) => {
                todo.mark(key, false)
                    .map_err(|e| format!("Invalid key {}", e))?;
                todo.save()?;
                Ok(())
            }
            None => Err("Key cannot be empty!".to_string())
        },

        "list" => {
            let (todo_items, done_items) = todo.list();
            println!("# TO DO");
            println!();
            todo_items.for_each(|x| println!(" * {}", x));

            println!();

            println!("# DONE");
            println!();

            done_items.for_each(|x| println!(" * {}", x));

            Ok(())
        }
        cmd => Err(format!("Command {} not recognised", cmd)),

    };

    match result {
        Err(e) => println!("ERROR: {}", e),
        Ok(_) => println!("SUCCESS"),
    }
    Ok(())

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_todo() {
        let todo = TodoList::new();
    }

    #[test]
    fn add_item() {
        let mut todo = TodoList::new();
        todo.add(String::from("Some thing to do"));
        assert_eq!(todo.items.get("Some thing to do"), Some(&true));
    }
    #[test]
    fn add_item_already_exist() {
        let mut todo = TodoList::new();
        todo.add(String::from("Some thing to do"));
        todo.add(String::from("Some thing to do"));
        assert_eq!(todo.items.get("Some thing to do"), Some(&true));
        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_item_does_not_change_value() {
        let mut todo = TodoList::new();
        todo.add(String::from("Some thing to do"));

        if let Some(x) = todo.items.get_mut("Some thing to do") {
            *x = false;
        }

        todo.add(String::from("Some thing to do"));
        assert_eq!(todo.items.get("Some thing to do"), Some(&false));
        assert_eq!(todo.items.len(), 1);

    }

    #[test]
    fn mark_item () {
        let mut todo = TodoList::new();
        todo.add(String::from("Some thing to do"));
        todo.mark(String::from("Some thing to do"), false);
        assert_eq!(todo.items.get("Some thing to do"), Some(&false));
        todo.mark(String::from("Some thing to do"), true);
        assert_eq!(todo.items.get("Some thing to do"), Some(&true));
    }

    #[test]
    fn mark_item_does_not_exist() {
        let mut todo = TodoList::new();
        assert_eq!(
            todo.mark(String::from("Some thing to do"), false),
            Err(String::from("Some thing to do"))
        );
    }

    #[test]
    fn list_items() {
        let mut todo = TodoList::new();
        todo.add(String::from("Some thing to do"));
        todo.add(String::from("Some thing else to do"));
        todo.add(String::from("Some thing done"));
        todo.mark(String::from("Some thing done"), false);

        let (todo_items, done_items) = todo.list();

        let todo_items : Vec<String> = todo_items.cloned().collect();
        let done_items : Vec<String> = done_items.cloned().collect();

        assert!(todo_items.iter().any(|e| e == "Some thing to do"));
        assert!(todo_items.iter().any(|e| e == "Some thing else to do"));
        assert_eq!(todo_items.len(), 2);
        assert!(done_items.iter().any(|e| e == "Some thing done"));
        assert_eq!(done_items.len(), 1);
    }

}
