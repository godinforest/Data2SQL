// src/core/extract/csv.rs

use std::fs::File;
use std::path::Path;
use csv::{Reader, StringRecord};
use serde_json::{Map, Value};

pub struct CsvExtractor {
    reader: Reader<File>,
    headers: StringRecord,
}

impl CsvExtractor {
    pub fn new(path: &Path) -> Result<Self, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)
            .map_err(|e| e.to_string())?;

        let headers = reader.headers().map_err(|e| e.to_string())?.clone();

        Ok(Self { reader, headers })
    }
}

impl Iterator for CsvExtractor {
    type Item = Result<Value, String>;

    fn next(&mut self) -> Option<Self::Item> {
        process_record(&mut self.reader, &self.headers)
    }
}

pub struct TsvExtractor {
    reader: Reader<File>,
    headers: StringRecord,
}

impl TsvExtractor {
    pub fn new(path: &Path) -> Result<Self, String> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_path(path)
            .map_err(|e| e.to_string())?;

        let headers = reader.headers().map_err(|e| e.to_string())?.clone();

        Ok(Self { reader, headers })
    }
}

impl Iterator for TsvExtractor {
    type Item = Result<Value, String>;

    fn next(&mut self) -> Option<Self::Item> {
        process_record(&mut self.reader, &self.headers)
    }
}

// Helper function to handle row reading for both CSV and TSV
fn process_record(
    reader: &mut Reader<File>,
    headers: &StringRecord,
) -> Option<Result<Value, String>> {
    let mut record = StringRecord::new();
    
    match reader.read_record(&mut record) {
        Ok(true) => {
            let mut map = Map::new();
            for (i, header) in headers.iter().enumerate() {
                let val = record.get(i).unwrap_or("");
                map.insert(header.to_string(), Value::String(val.to_string()));
            }
            Some(Ok(Value::Object(map)))
        }
        Ok(false) => None, // Reached end of file
        Err(e) => Some(Err(e.to_string())),
    }
}