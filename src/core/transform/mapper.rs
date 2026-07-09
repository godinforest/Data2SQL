// src/core/transform/mapper.rs

use serde_json::{Map, Value};

pub struct RecordMapper;

impl RecordMapper {
    pub fn map_record(record: Value) -> Map<String, Value> {
        match record {
            Value::Object(map) => {
                let mut flat_map = Map::new();
                for (key, value) in map {
                    flat_map.insert(key, Self::sanitize_value(&value));
                }
                flat_map
            }
            _ => {
                let mut map = Map::new();
                map.insert("value".to_string(), Self::sanitize_value(&record));
                map
            }
        }
    }

    fn sanitize_value(value: &Value) -> Value {
        match value {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => value.clone(),
            Value::Array(_) | Value::Object(_) => {
                // Complex types are serialized to strings for database storage
                Value::String(value.to_string())
            }
        }
    }
}