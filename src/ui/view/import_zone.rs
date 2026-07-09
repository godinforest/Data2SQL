// src/ui/view/import_zone.rs

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};

use crate::ui::messages::Message;
use crate::ui::state::{AppState, ImportQueueItem, ImportStatus};
use crate::ui::theme;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let title = text("Import").size(40).style(Color::WHITE);

    let sub_title_row = row![
        text("Supportable extentions").style(Color::from_rgb(0.8, 0.8, 0.8)),
        Space::with_width(Length::Fill),

    ]
    .width(Length::Fill);

    // Tags
    let ext_row = row![
        tag_button("JSON", false),
        tag_button("CSV", false),
        tag_button("TSV", false),
        tag_button("XML", false),
    ]
    .spacing(10);

    let drop_zone = if state.import_queue.is_empty() {
        container(
            column![
                text("+").size(30).style(Color::WHITE),
                Space::with_height(Length::Fill),
                text("Drag your file here")
                    .size(24)
                    .style(Color::WHITE)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                Space::with_height(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    text("+").size(30).style(Color::WHITE)
                ]
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15)
        .style(theme::drop_zone_empty)
    } else {
        let mut queue_column = column![].spacing(15);
        for item in &state.import_queue {
            queue_column = queue_column.push(render_queue_item(item, state.export_directory.is_some()));
        }

        container(scrollable(queue_column).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(theme::drop_zone_empty)
    };

    column![title, sub_title_row, ext_row, Space::with_height(10), drop_zone]
        .spacing(15)
        .padding(30)
        .into()
}

fn tag_button<'a>(label: &str, is_selected: bool) -> Element<'a, Message> {
    let appearance = if is_selected { theme::tag_selected } else { theme::tag_unselected };
    let text_color = if is_selected { Color::BLACK } else { Color::WHITE };

    container(text(label).size(14).style(text_color))
        .padding([4, 12])
        .style(appearance)
        .into()
}

fn render_queue_item(item: &ImportQueueItem, has_export_dir: bool) -> Element<'_, Message> {
    let filename = item.file_path.file_name().unwrap_or_default().to_string_lossy().into_owned();

    let header_row = row![text(filename).size(16).style(Color::WHITE)].spacing(10);

    let status_row = match &item.status {
        ImportStatus::Pending => {
            let msg = if has_export_dir { "Pending..." } else { "Awaiting export folder..." };
            row![
                text(msg).style(theme::COLOR_MUTED),
                Space::with_width(Length::Fill),
                button("Cancel").on_press(Message::CancelImport(item.file_path.clone()))
            ]
        }
        ImportStatus::Processing(pct) => row![
            text(format!("Processing {:.0}%", pct * 100.0)).style(theme::COLOR_ACCENT),
            Space::with_width(Length::Fill),
            button("Cancel").on_press(Message::CancelImport(item.file_path.clone()))
        ],
        ImportStatus::Canceled => row![
            text("Canceled").style(theme::COLOR_MUTED),
            Space::with_width(Length::Fill),
            button("Retry").on_press(Message::RetryImport(item.file_path.clone()))
        ],
        ImportStatus::Failed(_err) => row![
            text("Failed").style(theme::COLOR_ERROR),
            Space::with_width(Length::Fill),
            button("Clear").on_press(Message::RemoveFailedFile(item.file_path.clone()))
        ],
    }
    .align_items(Alignment::Center)
    .width(Length::Fill);

    container(column![header_row, status_row].spacing(5).width(Length::Fill))
        .padding(10)
        .into()
}