// src/main.rs

mod app;
mod core;
mod ui;

use app::DataPumpApp;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    
    // Configure default window properties
    settings.window = iced::window::Settings {
        size: iced::Size::new(900.0, 650.0),
        min_size: Some(iced::Size::new(800.0, 600.0)),
        resizable: true,
        ..iced::window::Settings::default()
    };

    // Initialize and run the application GUI loop
    DataPumpApp::run(settings)
}