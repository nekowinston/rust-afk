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

pub struct AfkFont<'a> {
    pub font: &'a Font,
    italic: bool,
    bold: bool,
}

impl<'a> AfkFont<'a> {
    pub fn new() -> Self {
        Self {
            font: &FONT_REGULAR,
            italic: false,
            bold: false,
        }
    }
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
        self.update_font();
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
        self.update_font();
    }

    pub fn update_font(&mut self) {
        self.font = if self.italic {
            if self.bold {
                &FONT_BOLD_ITALIC
            } else {
                &FONT_ITALIC
            }
        } else if self.bold {
            &FONT_BOLD_REGULAR
        } else {
            &FONT_REGULAR
        }
    }
}

impl std::fmt::Debug for AfkFont<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AfkFont")
            .field("italic", &self.italic)
            .field("bold", &self.bold)
            .finish()
    }
}
