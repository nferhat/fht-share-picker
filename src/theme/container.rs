use iced::widget::container::{Catalog, Style};
use iced::Color;

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(secondary)
    }

    fn style(&self, item: &Self::Class<'_>) -> Style {
        item(self)
    }
}

/// Create a new container with the primary background color.
pub fn primary(theme: &Theme) -> Style {
    Style::default().with_background(theme.background.primary)
}

/// Create a new container with the secondary background color.
pub fn secondary(theme: &Theme) -> Style {
    Style::default().with_background(theme.background.secondary)
}

/// Create a new container with the tertiary background color.
pub fn tertiary(theme: &Theme) -> Style {
    Style::default().with_background(theme.background.tertiary)
}

/// Create a new container with a transparent background and a border.
pub fn bordered(theme: &Theme) -> Style {
    Style::default()
        .with_background(Color::TRANSPARENT)
        .with_border(theme.separator, 2f32)
}
