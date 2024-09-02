use std::{collections::HashMap, io::{self, Write}};
fn main() {
    let mut  inventory = Inventory::new(); 
    loop{
        println!("1. Add an item");
        println!("2. Update and item");
        println!("3. List an item");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().expect("failed to flush the stdout");

        let mut take_input: String = String::new();
        io::stdin().read_line(&mut take_input).expect("Failed to read the line");

        let choice:u8 = take_input.trim().parse().expect("failed to convert to int");

        match choice {
            1 => 
            {
                let (name, quantity) = enter_choice();
                inventory.add_item(name.to_string(),quantity);
            },
            2 => 
            {
                let (name, quantity) = enter_choice();
                inventory.update_item(name, quantity);
            },
            
            3 => inventory.list_item(),
            4 => break,
            _=> println!("You Entered a bad choice"),
        }
    }
}


fn enter_choice()->(String, u8){
    print!("Enter the Item name: ");
    io::stdout().flush().expect("failed to flush");

    let mut take_name:String = String::new();
    io::stdin().read_line(&mut take_name).expect("failed to read the name");
    let name:&str = take_name.trim();

    print!("Enter the Item Quantity: ");
    io::stdout().flush().expect("failed to flush");

    let mut take_quanity: String = String::new();
    io::stdin().read_line(&mut take_quanity).expect("failed to read the quantiy");
    let quantity:u8 = take_quanity.trim().parse().expect("failed to convert in integer");

    (name.to_string(), quantity)


}


struct Item {
    _name: String,
    _quantity: u8,
}
struct Inventory{
    _items: HashMap<String, Item>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
        _items: HashMap::new(),
        }
    }
    fn add_item(&mut self, name: String, quantity:u8){
        let item = Item{
            _name:name.to_string(),
            _quantity:quantity
        };
        if !self._items.contains_key(&name){
            self._items.insert(name.to_string(), item);
            println!("Item: {}  with quantity: {} is added successfully", name, quantity)
        }else{
            println!("Key '{}' already exists! Not inserting.", name);
        }

    }
    fn update_item(&mut self, name: String, quantity:u8){
        if let Some(item) = self._items.get_mut(&name){
            item._quantity = quantity;
            println!("Item: {}  with quantity: {} is updated successfully", name, quantity)
        }
        else{
            println!("No item present in Inventory with this key ");
        }
        
    }
    fn list_item(&self){
        if self._items.is_empty() {
            println!("No items in the inventory");
        }
        else{
            for (key, value) in &self._items {
                println!("Name: {} -- Quantity:{}", key, value._quantity);
            }
        }
    }
}