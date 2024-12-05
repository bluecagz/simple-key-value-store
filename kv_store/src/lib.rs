use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Serialize, Deserialize)]
pub struct KVStore {
    storage: HashMap<String, String>,
    file_path: String,
}

impl KVStore {
    pub fn new(file_path: &str) -> Self {
        let mut kv = KVStore {
            storage: HashMap::new(),
            file_path: file_path.to_string(),
        };
        kv.load_from_file();
        kv
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
        self.save_to_file();
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.storage.remove(key);
        self.save_to_file();
    }

    pub fn compact(&mut self) {
        self.save_to_file();
    }

    fn save_to_file(&self) {
        let file = File::create(&self.file_path).expect("Failed to create file");
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.storage).expect("Failed to serialize data");
    }

    fn load_from_file(&mut self) {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            if let Ok(data) = bincode::deserialize_from(reader) {
                self.storage = data;
            }
        }
    }
}
