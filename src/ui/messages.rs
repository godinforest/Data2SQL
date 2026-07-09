// src/ui/messages.rs

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    // System & Window Events
    WindowResized(u32, u32),

    // Import Zone
    FileDropped(PathBuf),
    RemoveFromQueue(PathBuf),
    CancelImport(PathBuf),
    RetryImport(PathBuf),

    // Extension Override & Modal
    OpenExtensionSelector(PathBuf),
    CloseExtensionSelector,
    SelectNewExtension(String),
    ConfirmExtensionOverride,
    RemoveFailedFile(PathBuf),

    // Export Zone
    SelectExportDirectory,
    ExportDirectorySelected(Option<PathBuf>),
    OpenFolder(PathBuf),

    // ETL Worker Events
    ProcessQueue,
    ImportProgress {
        file_path: PathBuf,
        percentage: f32,
    },
    ImportCompleted {
        file_path: PathBuf,
        output_path: PathBuf,
        duration_ms: u64,
    },
    ImportFailed {
        file_path: PathBuf,
        error_message: String,
    },
}