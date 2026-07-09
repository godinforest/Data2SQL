// src/core/task.rs

use std::path::PathBuf;
use std::time::Instant;
use iced::Command;

use crate::ui::messages::Message;
use crate::core::telemetry::{log_event, TelemetryEvent};
use crate::core::extract::factory::ReaderFactory;
use crate::core::transform::mapper::RecordMapper;
use crate::core::load::sqlite::SqliteLoader;

pub async fn execute_import(file_path: PathBuf, output_dir: PathBuf, extension: String) -> Message {
    log_event(TelemetryEvent::ImportStarted { 
        extension: extension.clone() 
    });

    let start_time = Instant::now();
    let file_stem = file_path.file_stem().unwrap_or_default().to_string_lossy();

    // Executing directly ensures no tokio runtime context errors
    let reader_res = ReaderFactory::get_reader(&file_path, Some(&extension));
    if let Err(e) = reader_res {
        return Message::ImportFailed { file_path, error_message: e };
    }
    let reader = reader_res.unwrap();

    let loader_res = SqliteLoader::new(&output_dir, &file_stem);
    if let Err(e) = loader_res {
        return Message::ImportFailed { file_path, error_message: e };
    }
    let mut loader = loader_res.unwrap();

    let mut batch = Vec::new();
    for result in reader {
        match result {
            Ok(value) => {
                batch.push(RecordMapper::map_record(value));
                if batch.len() >= 1000 {
                    if let Err(e) = loader.insert_records(&batch) {
                        return Message::ImportFailed { file_path, error_message: e };
                    }
                    batch.clear();
                }
            }
            Err(e) => {
                return Message::ImportFailed { file_path, error_message: e };
            }
        }
    }

    if !batch.is_empty() {
        if let Err(e) = loader.insert_records(&batch) {
            return Message::ImportFailed { file_path, error_message: e };
        }
    }

    match loader.finish() {
        Ok(output_path) => {
            let duration_ms = start_time.elapsed().as_millis() as u64;
            log_event(TelemetryEvent::ImportCompleted {
                extension,
                duration_ms,
            });
            Message::ImportCompleted {
                file_path,
                output_path,
                duration_ms,
            }
        }
        Err(error_message) => {
            log_event(TelemetryEvent::ImportFailed {
                extension,
                error_code: error_message.clone(),
            });
            Message::ImportFailed { file_path, error_message }
        }
    }
}

pub fn spawn_import_task(file_path: PathBuf, output_dir: PathBuf, extension: String) -> Command<Message> {
    Command::perform(
        execute_import(file_path, output_dir, extension),
        |message| message
    )
}