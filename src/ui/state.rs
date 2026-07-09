// src/ui/state.rs

#![allow(dead_code)]

use std::path::PathBuf;
use iced::{Command, Subscription};
use iced::event::{self, Event};
use iced::window::Event as WindowEvent;
use crate::ui::messages::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum ImportStatus {
    Pending,
    Processing(f32),
    Canceled,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ImportQueueItem {
    pub file_path: PathBuf,
    pub extension: Option<String>,
    pub status: ImportStatus,
}

#[derive(Debug, Clone)]
pub struct ExportHistoryItem {
    pub original_file: PathBuf,
    pub output_path: PathBuf,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct ModalState {
    pub is_open: bool,
    pub target_file: Option<PathBuf>,
    pub selected_extension: Option<String>,
}

impl Default for ModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            target_file: None,
            selected_extension: None,
        }
    }
}

pub struct AppState {
    pub import_queue: Vec<ImportQueueItem>,
    pub export_history: Vec<ExportHistoryItem>,
    pub export_directory: Option<PathBuf>,
    pub extension_modal: ModalState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            import_queue: Vec::new(),
            export_history: Vec::new(),
            export_directory: None,
            extension_modal: ModalState::default(),
        }
    }
}

impl AppState {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::WindowResized(_, _) => Command::none(),
            
            Message::FileDropped(path) => {
                let extension = path.extension().map(|ext| ext.to_string_lossy().into_owned());
                self.import_queue.push(ImportQueueItem {
                    file_path: path,
                    extension,
                    status: ImportStatus::Pending,
                });
                
                Command::perform(async {}, |_| Message::ProcessQueue)
            }
            
            Message::RemoveFromQueue(path) => {
                self.import_queue.retain(|item| item.file_path != path);
                Command::none()
            }
            
            Message::CancelImport(path) => {
                if let Some(item) = self.import_queue.iter_mut().find(|i| i.file_path == path) {
                    item.status = ImportStatus::Canceled;
                }
                Command::none()
            }
            
            Message::RetryImport(path) => {
                if let Some(item) = self.import_queue.iter_mut().find(|i| i.file_path == path) {
                    item.status = ImportStatus::Pending;
                }
                Command::perform(async {}, |_| Message::ProcessQueue)
            }
            
            Message::OpenExtensionSelector(path) => {
                self.extension_modal.is_open = true;
                self.extension_modal.target_file = Some(path);
                self.extension_modal.selected_extension = None;
                Command::none()
            }
            
            Message::CloseExtensionSelector => {
                self.extension_modal = ModalState::default();
                Command::none()
            }
            
            Message::SelectNewExtension(ext) => {
                self.extension_modal.selected_extension = Some(ext);
                Command::none()
            }
            
            Message::ConfirmExtensionOverride => {
                if let (Some(path), Some(ext)) = (
                    self.extension_modal.target_file.clone(),
                    self.extension_modal.selected_extension.clone(),
                ) {
                    if let Some(item) = self.import_queue.iter_mut().find(|i| i.file_path == path) {
                        item.extension = Some(ext);
                        item.status = ImportStatus::Pending;
                    }
                }
                self.extension_modal = ModalState::default();
                Command::perform(async {}, |_| Message::ProcessQueue)
            }
            
            Message::RemoveFailedFile(path) => {
                self.import_queue.retain(|item| item.file_path != path);
                Command::none()
            }
            
            Message::SelectExportDirectory => {
                Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::ExportDirectorySelected
                )
            }
            
            Message::ExportDirectorySelected(path) => {
                if path.is_some() {
                    self.export_directory = path;
                    return Command::perform(async {}, |_| Message::ProcessQueue);
                }
                Command::none()
            }
            
            Message::OpenFolder(path) => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer").arg(path).spawn().ok();
                
                #[cfg(target_os = "macos")]
                std::process::Command::new("open").arg(path).spawn().ok();
                
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open").arg(path).spawn().ok();

                Command::none()
            }
            
            Message::ProcessQueue => {
                let mut commands = Vec::new();
                if let Some(export_dir) = &self.export_directory {
                    for item in self.import_queue.iter_mut().filter(|i| i.status == ImportStatus::Pending) {
                        let ext = item.extension.clone().unwrap_or_else(|| "json".to_string());
                        
                        commands.push(crate::core::task::spawn_import_task(
                            item.file_path.clone(),
                            export_dir.clone(),
                            ext,
                        ));
                        
                        item.status = ImportStatus::Processing(0.0);
                    }
                }
                Command::batch(commands)
            }
            
            Message::ImportProgress { file_path, percentage } => {
                if let Some(item) = self.import_queue.iter_mut().find(|i| i.file_path == file_path) {
                    item.status = ImportStatus::Processing(percentage);
                }
                Command::none()
            }
            
            Message::ImportCompleted { file_path, output_path, .. } => {
                self.import_queue.retain(|item| item.file_path != file_path);
                
                self.export_history.insert(0, ExportHistoryItem {
                    original_file: file_path,
                    output_path,
                    success: true,
                });
                
                self.export_history.truncate(10);
                Command::none()
            }
            
            Message::ImportFailed { file_path, error_message } => {
                if let Some(item) = self.import_queue.iter_mut().find(|i| i.file_path == file_path) {
                    item.status = ImportStatus::Failed(error_message);
                }
                Command::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status| match event {
            Event::Window(_, WindowEvent::FileDropped(path)) => {
                Some(Message::FileDropped(path))
            }
            _ => None,
        })
    }
}