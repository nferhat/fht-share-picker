use std::borrow::Cow;

// Font declarations
pub static MONO: iced::Font = iced::Font::with_name("Fht Mono");
pub static MONO_BOLD: iced::Font = iced::Font {
    weight: iced::font::Weight::Bold,
    ..iced::Font::with_name("Fht Mono")
};

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../assets/fonts/FhtMono-Regular.ttf")
            .as_slice()
            .into(),
        include_bytes!("../assets/fonts/FhtMono-Bold.ttf")
            .as_slice()
            .into(),
    ]
}
