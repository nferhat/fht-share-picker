use iced::{color, Color};

pub mod container;
pub mod text;

// The UI theme.
#[allow(unused)]
pub struct Theme {
    /// The background for the UI.
    pub background: Palette,
    /// The main text color for the UI.
    pub text: Palette,
    /// The accent color for important text.
    pub accent: Color,
    /// Text color for error messages.
    pub error: Color,
    /// The color for seperators of UI elements, or enclosures.
    pub separator: Color,
    /// Text color for warning messages.
    pub warning: Color,
    /// Text color for info messages.
    pub info: Color,
    /// The eight standard terminal ansi colors.
    pub ansi: Ansi,
    /// The eight bright terminal ansi colors.
    pub ansi_bright: Ansi,
}

/// A single palette for a UI.
#[allow(unused)]
pub struct Palette {
    /// The primary, or main color, used for large areas.
    pub primary: Color,
    /// The secondary color, for other less important UI elements, that should indicate updates or
    /// new information to the user, and may require user interaction.
    pub secondary: Color,
    /// The tertiary color, for miscellaneous UI elements that should not distract the user's main
    /// interaction with the other part of the UI.
    ///
    /// Can also be used to create contrast with the secondary UI color.
    pub tertiary: Color,
}

/// The eight terminal ansi colors.
#[allow(unused)]
pub struct Ansi {
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub black: Color,
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Self {
            background: Palette {
                primary: color!(0x101115),
                secondary: color!(0x0e0f13),
                tertiary: color!(0x131419),
            },
            text: Palette {
                primary: color!(0xc4c4c4),
                secondary: color!(0xc4c6d0),
                tertiary: color!(0x53536a),
            },
            accent: color!(0x6791c9),
            error: color!(0xdf5b61),
            warning: color!(0xde8f78),
            info: color!(0x87c7a1),
            separator: color!(0x222230),
            ansi: Ansi {
                black: color!(0x14161f),
                red: color!(0xdf5b61),
                green: color!(0x87c7a1),
                yellow: color!(0xde8f78),
                blue: color!(0x6791c9),
                magenta: color!(0xbc83e3),
                cyan: color!(0x70b9cc),
                white: color!(0xc4c4c4),
            },
            ansi_bright: Ansi {
                black: color!(0x161922),
                red: color!(0xee6a70),
                green: color!(0x96d6b0),
                yellow: color!(0xffb29b),
                blue: color!(0x7ba5dd),
                magenta: color!(0xcb92f2),
                cyan: color!(0x7fc8db),
                white: color!(0xcccccc),
            },
        }
    }
}

impl iced::daemon::DefaultStyle for Theme {
    fn default_style(&self) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            text_color: self.text.primary,
            background_color: self.background.primary,
        }
    }
}
