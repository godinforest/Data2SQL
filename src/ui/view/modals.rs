// src/ui/view/modals.rs

use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

use crate::ui::messages::Message;
use crate::ui::state::AppState;
use crate::ui::theme;

pub fn view<'a>(state: &'a AppState, _base: Element<'a, Message>) -> Element<'a, Message> {
    let mut extensions_col = column![].spacing(10);
    
    let ext_list = ["JSON", "CSV", "TSV", "XML"];
    
    for ext in ext_list {
        let is_selected = state.extension_modal.selected_extension.as_deref() == Some(ext);
        
        let mut btn_text = text(ext).size(16);
        
        if is_selected {
            btn_text = btn_text.style(theme::COLOR_ACCENT);
        } else {
            btn_text = btn_text.style(Color::WHITE);
        }
            
        let btn = button(
            container(btn_text)
                .width(Length::Fill)
                .center_x()
        )
        .style(iced::theme::Button::Text)
        .width(Length::Fill)
        .on_press(Message::SelectNewExtension(ext.to_string()));
        
        extensions_col = extensions_col.push(btn);
    }

    let confirm_btn = button(
        text("Confirm")
            .size(16)
            .style(theme::COLOR_SUCCESS)
    )
    .style(iced::theme::Button::Text)
    .on_press(Message::ConfirmExtensionOverride);
        
    let cancel_btn = button(
        text("Cancel")
            .size(16)
            .style(theme::COLOR_MUTED)
    )
    .style(iced::theme::Button::Text)
    .on_press(Message::CloseExtensionSelector);
        
    let controls_row = row![
        cancel_btn, 
        Space::with_width(Length::Fill), 
        confirm_btn
    ]
    .align_items(Alignment::Center)
    .width(Length::Fill);

    let modal_card = container(
        column![
            text("Choose new data type")
                .size(24)
                .style(Color::WHITE),
            extensions_col,
            controls_row
        ]
        .spacing(20)
    )
    .padding(30)
    .width(Length::Fixed(350.0))
    .style(modal_card_style);

    container(modal_card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(modal_backdrop_style)
        .into()
}

// Use explicit named functions for styles to avoid Rust closure lifetime inference errors
fn modal_card_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.12))),
        border: Border {
            color: Color::WHITE,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

fn modal_backdrop_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
        ..Default::default()
    }
}