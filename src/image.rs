use crate::fonts::{FONT_BOLD_ITALIC, FONT_BOLD_REGULAR, FONT_ITALIC, FONT_REGULAR};
use catppuccin::{ColorName, FlavorName, PALETTE};
use pulldown_cmark::{Event, Tag, TagEnd};
use ril::{text::TextLayout, Draw, Font, Image, Rgba};

const PADDING: (u32, u32) = (200, 200);

struct AfkFont<'a> {
    pub font: &'a Font,
    italic: bool,
    bold: bool,
}

impl std::fmt::Debug for AfkFont<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AfkFont")
            .field("italic", &self.italic)
            .field("bold", &self.bold)
            .finish()
    }
}

impl<'a> AfkFont<'a> {
    fn new() -> Self {
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

pub fn create(text: &str, flavor: FlavorName, color: ColorName) -> Image<Rgba> {
    let foreground = ctp_rgb_to_ril_rgba(PALETTE[flavor][color].rgb);
    let background = ctp_rgb_to_ril_rgba(PALETTE[flavor][ColorName::Base].rgb);

    // having 0 width or height will cause a panic, so just return a blank image
    if text.is_empty() {
        return Image::new(PADDING.0, PADDING.1, background);
    }

    let mut layout = TextLayout::new();

    let parser = pulldown_cmark::Parser::new(text);
    let mut font = AfkFont::new();
    let mut codefence: Option<String> = None;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Emphasis => {
                    font.set_italic(true);
                }
                Tag::Strong => {
                    font.set_bold(true);
                }
                Tag::CodeBlock(kind) => {
                    if let pulldown_cmark::CodeBlockKind::Fenced(fence) = kind {
                        codefence = Some(fence.to_string());
                    }
                    layout.push_basic_text(font.font, "\n", foreground);
                }
                _ => (),
            },
            Event::Text(text) => {
                dbg!(&font, &text);
                if codefence.is_some() {
                    layout.push_basic_text(
                        font.font,
                        text,
                        ctp_rgb_to_ril_rgba(PALETTE[flavor][ColorName::Text].rgb),
                    );
                    codefence = None;
                } else {
                    layout.push_basic_text(font.font, text, foreground);
                }
            }
            Event::End(tag) => match tag {
                TagEnd::Emphasis => {
                    font.set_italic(false);
                }
                TagEnd::Strong => {
                    font.set_bold(false);
                }
                _ => (),
            },
            Event::SoftBreak | Event::HardBreak => {
                layout.push_basic_text(font.font, "\n", foreground);
            }
            _ => (),
        }
    }

    let (mut width, mut height) = layout.dimensions();

    if width == 0 {
        width = 1;
    }
    if height == 0 {
        height = 1;
    }

    let mut text_image = Image::new(width, height, Rgba::transparent());
    layout.draw(&mut text_image);

    let mut img = Image::new(width + PADDING.0, height + PADDING.1, background)
        .with_overlay_mode(ril::OverlayMode::Merge);

    img.paste(PADDING.0 / 2, PADDING.1 / 2, &text_image);

    img
}

const fn ctp_rgb_to_ril_rgba(rgb: catppuccin::Rgb) -> ril::Rgba {
    Rgba {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: 255,
    }
}
