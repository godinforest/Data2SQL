// src/core/extract/json.rs

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::{Value, Map};

pub struct JsonExtractor {
    pub headers: Vec<String>,
    items: std::vec::IntoIter<Value>,
}

impl JsonExtractor {
    pub fn new(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        
        // Parse the entire file into a root JSON value.
        let root: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
        
        let items = match root {
            Value::Array(arr) => arr,
            _ => vec![root],
        };

        let mut headers = Vec::new();

        // Extract headers from the keys of the first JSON object.
        if let Some(first_item) = items.first() {
            if let Value::Object(map) = first_item {
                headers = map.keys().cloned().collect();
            } else {
                return Err("The first JSON element is not an object. Cannot map database headers.".to_string());
            }
        }

        Ok(Self {
            headers,
            items: items.into_iter(),
        })
    }
}

impl Iterator for JsonExtractor {
    // Yield Result<Value, String> to match RecordStream in factory.rs
    type Item = Result<Value, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.items.next()?;
        
        match next_item {
            Value::Object(mut map) => {
                let mut row_map = Map::new();
                
                // Filter the object to only include the established headers.
                // Missing keys will be inserted as Value::Null.
                for header in &self.headers {
                    let value = map.remove(header).unwrap_or(Value::Null);
                    row_map.insert(header.clone(), value);
                }
                
                // Return as Value::Object to satisfy the signature
                Some(Ok(Value::Object(row_map)))
            }
            _ => Some(Err("Encountered a non-object JSON value during iteration.".to_string())),
        }
    }
}