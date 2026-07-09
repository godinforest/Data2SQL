// src/ui/view/mod.rs

pub mod export_zone;
pub mod import_zone;
pub mod modals;

use iced::widget::{Container, Row};
use iced::{Element, Length};

use crate::ui::messages::Message;
use crate::ui::state::AppState;
use crate::ui::theme;

pub fn render_main(state: &AppState) -> Element<'_, Message> {
    let content = Row::new()
        .push(
            Container::new(import_zone::view(state))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(theme::left_panel_black),
        )
        .push(
            Container::new(export_zone::view(state))
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(theme::right_panel_white),
        )
        .height(Length::Fill);

    let base_layout: Element<'_, Message> = Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    if state.extension_modal.is_open {
        modals::view(state, base_layout)
    } else {
        base_layout
    }
}