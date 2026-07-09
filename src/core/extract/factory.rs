// src/core/extract/factory.rs

use std::path::Path;
use serde_json::Value;

use super::csv::{CsvExtractor, TsvExtractor};
use super::json::JsonExtractor;
use super::xml::XmlExtractor;

// RecordStream expects Result<Value, String>
pub type RecordStream = Box<dyn Iterator<Item = Result<Value, String>> + Send>;

pub struct ReaderFactory;

impl ReaderFactory {
    pub fn get_reader(path: &Path, override_extension: Option<&str>) -> Result<RecordStream, String> {
        let ext = override_extension
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| {
                path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default()
                    .to_lowercase()
            });

        match ext.as_str() {
            "json" => {
                let extractor = JsonExtractor::new(path)?;
                Ok(Box::new(extractor))
            }
            "csv" => {
                let extractor = CsvExtractor::new(path)?;
                Ok(Box::new(extractor))
            }
            "tsv" => {
                let extractor = TsvExtractor::new(path)?;
                Ok(Box::new(extractor))
            }
            "xml" => {
                let extractor = XmlExtractor::new(path)?;
                Ok(Box::new(extractor))
            }
            _ => Err(format!("Unsupported file format: {}", ext)),
        }
    }
}