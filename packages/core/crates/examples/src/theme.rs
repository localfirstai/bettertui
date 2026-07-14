//! Theme using BetterTUI's Color types.
//!
//! Demonstrates proper color handling with `Color::rgb()` and `Color::Named()`.

use bettertui_engine::tree::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub title_color: Color,
    pub border_color: Color,
    pub focused_border_color: Color,
    pub text_color: Color,
    pub category_color: Color,
    pub selected_bg_color: Color,
    pub selected_text_color: Color,
    pub description_color: Color,
    pub instructions_color: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            title_color: Color::rgb(240, 248, 255),
            border_color: Color::rgb(71, 85, 105),
            focused_border_color: Color::rgb(96, 165, 250),
            text_color: Color::rgb(226, 232, 240),
            category_color: Color::rgb(148, 163, 184),
            selected_bg_color: Color::rgb(30, 58, 95),
            selected_text_color: Color::rgb(56, 189, 248),
            description_color: Color::rgb(100, 116, 139),
            instructions_color: Color::rgb(148, 163, 184),
        }
    }

    pub fn light() -> Self {
        Self {
            title_color: Color::rgb(15, 23, 42),
            border_color: Color::rgb(203, 213, 225),
            focused_border_color: Color::rgb(37, 99, 235),
            text_color: Color::rgb(15, 23, 42),
            category_color: Color::rgb(71, 85, 105),
            selected_bg_color: Color::rgb(219, 234, 254),
            selected_text_color: Color::rgb(29, 78, 216),
            description_color: Color::rgb(71, 85, 105),
            instructions_color: Color::rgb(71, 85, 105),
        }
    }
}
