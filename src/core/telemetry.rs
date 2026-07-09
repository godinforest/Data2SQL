// src/core/telemetry.rs

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    AppStarted,
    FileDropped { extension: String },
    ExtensionOverridden { from: String, to: String },
    ImportStarted { extension: String },
    ImportCompleted { extension: String, duration_ms: u64 },
    ImportFailed { extension: String, error_code: String },
    ImportCanceled { extension: String },
}

#[derive(Debug, Serialize)]
struct LogEntry {
    timestamp: String,
    event: TelemetryEvent,
}

pub fn log_event(event: TelemetryEvent) {
    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        event,
    };

    if let Ok(json_line) = serde_json::to_string(&entry) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("telemetry.jsonl");

        if let Ok(mut f) = file {
            let _ = writeln!(f, "{}", json_line);
        }
    }
}