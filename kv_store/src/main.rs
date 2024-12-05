use std::io::{self, Write};
use simple_key_value_store::KVStore;

fn main() {
    let mut kv = KVStore::new("store.bin");
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "set" if parts.len() == 3 => kv.set(parts[1].to_string(), parts[2].to_string()),
            "get" if parts.len() == 2 => {
                if let Some(value) = kv.get(parts[1]) {
                    println!("{}", value);
                } else {
                    println!("Key not found");
                }
            }
            "delete" if parts.len() == 2 => kv.delete(parts[1]),
            "compact" => kv.compact(),
            "exit" => break,
            _ => println!("Commands: set <key> <value>, get <key>, delete <key>, compact, exit"),
        }
    }
}
