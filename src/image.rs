use crate::fonts::AfkFont;
use crate::highlight::highlight;

use catppuccin::{ColorName, FlavorName, PALETTE};
use pulldown_cmark::{Event, Tag, TagEnd};
use ril::{text::TextLayout, Draw, Image, Rgba};

const PADDING: (u32, u32) = (200, 200);

pub fn create(text: &str, flavor: FlavorName, color: ColorName) -> Image<Rgba> {
    let foreground = ctp_rgb_to_ril_rgba(flavor, color);
    let background = ctp_rgb_to_ril_rgba(flavor, ColorName::Base);

    // having 0 width or height will cause a panic, so just return a blank image
    if text.is_empty() {
        return Image::new(PADDING.0, PADDING.1, background);
    }

    let mut layout = TextLayout::new();

    let parser = pulldown_cmark::Parser::new(text);
    let mut font = AfkFont::new();
    let mut codefence: Option<String> = None;

    for event in parser {
        dbg!(&event);
        match event {
            Event::Start(tag) => match tag {
                Tag::Emphasis => font.set_italic(true),
                Tag::Strong => font.set_bold(true),
                Tag::CodeBlock(kind) => {
                    if let pulldown_cmark::CodeBlockKind::Fenced(fence) = kind {
                        codefence = Some(fence.to_string());
                    }
                    layout.push_basic_text(font.font, "\n", foreground);
                }
                _ => (),
            },
            Event::Text(text) => {
                if codefence.is_some() {
                    highlight(
                        &mut layout,
                        &codefence.unwrap(),
                        &text.into_string(),
                        flavor,
                    );
                    codefence = None;
                } else {
                    layout.push_basic_text(font.font, text, foreground);
                }
            }
            Event::End(tag) => match tag {
                TagEnd::Emphasis => font.set_italic(false),
                TagEnd::Strong => font.set_bold(false),
                TagEnd::Paragraph => layout.push_basic_text(font.font, "\n", foreground),
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

fn ctp_rgb_to_ril_rgba(flavor: FlavorName, color: ColorName) -> ril::Rgba {
    let rgb = PALETTE[flavor][color].rgb;
    Rgba {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: 255,
    }
}
