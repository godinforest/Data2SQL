// src/app.rs

use iced::{Application, Command, Element, Theme};
use crate::ui::messages::Message;
use crate::ui::state::AppState;

pub struct DataPumpApp {
    state: AppState,
}

impl Application for DataPumpApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            DataPumpApp {
                state: AppState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Data Pump ETL")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.state.update(message)
    }

    fn view(&self) -> Element<'_, Message> {
        // Route the view rendering to the UI module
        crate::ui::view::render_main(&self.state)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // Essential: Link the state's event listeners to the main app loop
        self.state.subscription()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}