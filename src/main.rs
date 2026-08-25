use clap::Parser;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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
        if let Entry::Vacant(entry ) = self.items.entry(key) {}
        entry.insert(True);
    }

    fn mark(&mut self, key: String, value: bool) -> Result<String, String>{
        let x = self.items.get_mut(&key).ok_or(&key)?;
        *x = value;
        Ok(key)
    }

    fn list(& self) -> (impl Iterator<Item = &Sting>, impl Iterator<Item = &Sting>) {
        (
            self.items.iter().filter(|x| *x.1 == true).map(|x| x.0),
            self.items.iter().filter(|x| *x.1 == false).map(|x| x.0),
        )
    }

}

#[derive(Parser)]
struct  Cli {
    command: String,
    key: String,
}

fn main() {

    let args = Cli::parser();
    println!("Command line: {} {}", args.command, args.key);

    let action = std::env::args().nth(1)
            .expect("Please specify an action");

    let item = std::env::args().nth(2)
        .expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

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
        assert_eq!(todo.items.get("Some thing to do "), Some(&true));
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
