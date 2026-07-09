// src/ui/view/export_zone.rs

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};
use std::path::Path;

use crate::ui::messages::Message;
use crate::ui::state::AppState;
use crate::ui::theme;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let title_row = row![
        text("Export").size(40).style(Color::BLACK),
        Space::with_width(Length::Fill),
        
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center);

    let dir_text = match &state.export_directory {
        Some(path) => format_short_path(path),
        None => "Choose directory".to_string(),
    };

    let choose_dir_btn = button(text(dir_text).style(theme::COLOR_ACCENT))
        .style(iced::theme::Button::Text)
        .on_press(Message::SelectExportDirectory);

    let show_folder_btn = match &state.export_directory {
        Some(path) => {
            button(
                container(text("Show Folder").style(Color::WHITE))
                    .width(Length::Fill)
                    .center_x()
            )
            .width(Length::Fill)
            .style(iced::theme::Button::Primary)
            .on_press(Message::OpenFolder(path.clone()))
        }
        None => {
            button(
                container(text("Show Folder").style(Color::WHITE))
                    .width(Length::Fill)
                    .center_x()
            )
            .width(Length::Fill)
            .style(iced::theme::Button::Primary)
        }
    };

    let history_zone = if state.export_history.is_empty() {
        container(text("Export history").size(24).style(Color::BLACK))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(theme::history_zone)
    } else {
        let mut lists_column = column![].spacing(10);
        for item in &state.export_history {
            let filename = item.output_path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            lists_column = lists_column.push(
                container(
                    row![
                        text(filename).style(Color::BLACK),
                        Space::with_width(Length::Fill),
                        text("Done").style(theme::COLOR_SUCCESS)
                    ]
                    .align_items(Alignment::Center),
                )
                .padding(10),
            );
        }

        container(scrollable(lists_column).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(theme::history_zone)
    };

    column![title_row, choose_dir_btn, show_folder_btn, Space::with_height(10), history_zone]
        .spacing(15)
        .padding(30)
        .into()
}

fn format_short_path(path: &Path) -> String {
    let components: Vec<_> = path.components().collect();
    let len = components.len();
    if len <= 2 {
        path.to_string_lossy().into_owned()
    } else {
        let p1 = components[len - 2].as_os_str().to_string_lossy();
        let p2 = components[len - 1].as_os_str().to_string_lossy();
        format!(".../{}/{}", p1, p2)
    }
}