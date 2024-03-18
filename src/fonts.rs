use lazy_static::lazy_static;
use ril::Font;

const FONT_SIZE: f32 = 96.0;
lazy_static! {
    pub static ref FONT_REGULAR: Font = Font::from_bytes(
        include_bytes!("../assets/VictorMonoNerdFont-Medium.ttf"),
        FONT_SIZE
    )
    .unwrap();
    pub static ref FONT_ITALIC: Font = Font::from_bytes(
        include_bytes!("../assets/VictorMonoNerdFont-MediumItalic.ttf"),
        FONT_SIZE
    )
    .unwrap();
    pub static ref FONT_BOLD_REGULAR: Font = Font::from_bytes(
        include_bytes!("../assets/VictorMonoNerdFont-Bold.ttf"),
        FONT_SIZE
    )
    .unwrap();
    pub static ref FONT_BOLD_ITALIC: Font = Font::from_bytes(
        include_bytes!("../assets/VictorMonoNerdFont-BoldItalic.ttf"),
        FONT_SIZE
    )
    .unwrap();
}
