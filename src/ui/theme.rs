// src/ui/theme.rs

#![allow(dead_code)]

use iced::widget::container;
use iced::{Color, Theme, Background, Border};

// --- Theme Styles ---

pub fn left_panel_black(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::BLACK)),
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn right_panel_white(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::WHITE)),
        text_color: Some(Color::BLACK),
        ..Default::default()
    }
}

pub fn drop_zone_empty(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        border: Border {
            color: Color::WHITE,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn history_zone(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn tag_selected(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::WHITE)),
        text_color: Some(Color::BLACK),
        ..Default::default()
    }
}

pub fn tag_unselected(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::BLACK)),
        text_color: Some(Color::WHITE),
        border: Border {
            color: Color::WHITE,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub const COLOR_ACCENT: Color = Color::from_rgb(0.20, 0.50, 0.90);
pub const COLOR_SUCCESS: Color = Color::from_rgb(0.20, 0.80, 0.40);
pub const COLOR_ERROR: Color = Color::from_rgb(0.90, 0.30, 0.30);
pub const COLOR_MUTED: Color = Color::from_rgb(0.50, 0.50, 0.50);