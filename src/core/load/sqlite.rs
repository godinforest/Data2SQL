// src/core/load/sqlite.rs

use rusqlite::{Connection, types::Value as SqlValue};
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub struct SqliteLoader {
    conn: Option<Connection>,
    temp_path: PathBuf,
    final_path: PathBuf,
    table_created: bool,
    headers: Vec<String>,
}

impl SqliteLoader {
    pub fn new(output_dir: &Path, file_stem: &str) -> Result<Self, String> {
        let final_filename = format!("{}.db", file_stem);
        let temp_filename = format!("NotFinished_{}", final_filename);
        
        let temp_path = output_dir.join(&temp_filename);
        let final_path = output_dir.join(&final_filename);

        let conn = Connection::open(&temp_path).map_err(|e| e.to_string())?;

        Ok(Self {
            conn: Some(conn),
            temp_path,
            final_path,
            table_created: false,
            headers: Vec::new(),
        })
    }

    pub fn insert_records(&mut self, records: &[Map<String, Value>]) -> Result<(), String> {
        if records.is_empty() {
            return Ok(());
        }

        let conn = self.conn.as_mut().ok_or("Database connection is closed")?;

        // Create the table dynamically based on the keys of the first record batch
        if !self.table_created {
            self.headers = records[0].keys().cloned().collect();

            if self.headers.is_empty() {
                return Err("Record contains no keys to create database columns".to_string());
            }

            let mut columns_def = Vec::new();
            for header in &self.headers {
                // Wrap headers in quotes to prevent syntax errors with reserved SQL keywords
                columns_def.push(format!("\"{}\" TEXT", header));
            }

            let create_sql = format!(
                "CREATE TABLE IF NOT EXISTS records (id INTEGER PRIMARY KEY AUTOINCREMENT, {})",
                columns_def.join(", ")
            );

            conn.execute(&create_sql, []).map_err(|e| e.to_string())?;
            self.table_created = true;
        }

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        {
            let placeholders = vec!["?"; self.headers.len()].join(", ");
            let quoted_headers: Vec<String> = self.headers.iter().map(|h| format!("\"{}\"", h)).collect();
            let insert_sql = format!(
                "INSERT INTO records ({}) VALUES ({})",
                quoted_headers.join(", "),
                placeholders
            );

            let mut stmt = tx.prepare(&insert_sql).map_err(|e| e.to_string())?;

            for record in records {
                let mut sql_values: Vec<SqlValue> = Vec::with_capacity(self.headers.len());

                // Match map values to headers, defaulting to NULL if a key is missing
                for header in &self.headers {
                    let json_val = record.get(header).unwrap_or(&Value::Null);
                    
                    // Convert serde_json::Value to rusqlite native types
                    let sql_val = match json_val {
                        Value::Null => SqlValue::Null,
                        Value::Bool(b) => SqlValue::Integer(if *b { 1 } else { 0 }),
                        Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                SqlValue::Integer(i)
                            } else if let Some(f) = n.as_f64() {
                                SqlValue::Real(f)
                            } else {
                                SqlValue::Text(n.to_string())
                            }
                        }
                        Value::String(s) => SqlValue::Text(s.clone()),
                        Value::Array(a) => SqlValue::Text(serde_json::to_string(a).unwrap_or_default()),
                        Value::Object(o) => SqlValue::Text(serde_json::to_string(o).unwrap_or_default()),
                    };
                    
                    sql_values.push(sql_val);
                }

                let params: Vec<&dyn rusqlite::ToSql> = sql_values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
                stmt.execute(&*params).map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn finish(mut self) -> Result<PathBuf, String> {
        // Drop connection to release file locks before renaming the database file
        if let Some(conn) = self.conn.take() {
            let _ = conn.close();
        }
        
        fs::rename(&self.temp_path, &self.final_path).map_err(|e| e.to_string())?;
        
        Ok(self.final_path.clone())
    }
}