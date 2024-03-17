use catppuccin::{ColorName, FlavorName, PALETTE};
use lazy_static::lazy_static;
use ril::{text::TextLayout, Draw, Font, Image, Rgba};

const FONT_SIZE: f32 = 96.0;
const PADDING: (u32, u32) = (200, 200);

lazy_static! {
    static ref FONT_REGULAR: Font = Font::from_bytes(
        include_bytes!("../assets/Victor-Mono-SemiBold-NF.ttf"),
        FONT_SIZE
    )
    .unwrap();
    static ref FONT_ITALIC: Font = Font::from_bytes(
        include_bytes!("../assets/Victor-Mono-SemiBold-Italic-NF.ttf"),
        FONT_SIZE
    )
    .unwrap();
}

pub fn create_image(text: &str, italic: bool, flavor: FlavorName, color: ColorName) -> Image<Rgba> {
    let foreground = ctp_rgb_to_ril_rgba(PALETTE[flavor][color].rgb);
    let background = ctp_rgb_to_ril_rgba(PALETTE[flavor][ColorName::Base].rgb);

    let font: Font = if italic {
        FONT_ITALIC.to_owned()
    } else {
        FONT_REGULAR.to_owned()
    };

    let layout = TextLayout::new().with_basic_text(&font, text, foreground);

    let (width, height) = layout.dimensions();
    let mut text_image = Image::new(width, height, Rgba::transparent());
    layout.draw(&mut text_image);

    let mut img = Image::new(width + PADDING.0, height + PADDING.1, background)
        .with_overlay_mode(ril::OverlayMode::Merge);

    img.paste(PADDING.0 / 2, PADDING.1 / 2, &text_image);

    img
}

fn ctp_rgb_to_ril_rgba(rgb: catppuccin::Rgb) -> ril::Rgba {
    Rgba {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: 255,
    }
}
