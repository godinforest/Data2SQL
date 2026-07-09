// src/core/extract/xml.rs

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};

pub struct XmlExtractor {
    reader: Reader<BufReader<File>>,
    buf: Vec<u8>,
}

impl XmlExtractor {
    pub fn new(path: &Path) -> Result<Self, String> {
        let reader = Reader::from_file(path).map_err(|e| e.to_string())?;
        
        Ok(Self {
            reader,
            buf: Vec::new(),
        })
    }
}

impl Iterator for XmlExtractor {
    type Item = Result<Value, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buf.clear();
            
            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::Start(ref e)) => {
                    // This is a simplified extraction logic for the ETL skeleton.
                    // A production implementation would recursively read child tags 
                    // and attributes to build a complete JSON object representing the node.
                    let name = String::from_utf8_lossy(e.name().into_inner()).into_owned();
                    let mut map = Map::new();
                    
                    map.insert(
                        "element_name".to_string(), 
                        Value::String(name)
                    );
                    
                    return Some(Ok(Value::Object(map)));
                }
                Ok(Event::Empty(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().into_inner()).into_owned();
                    let mut map = Map::new();
                    
                    map.insert(
                        "element_name".to_string(), 
                        Value::String(name)
                    );
                    
                    return Some(Ok(Value::Object(map)));
                }
                Ok(Event::Eof) => return None,
                Err(e) => return Some(Err(e.to_string())),
                _ => continue, // Skip irrelevant events like whitespace or comments
            }
        }
    }
}