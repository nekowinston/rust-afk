use crate::fonts;

use catppuccin::FlavorName;
use lazy_static::lazy_static;
use ril::{Rgba, TextLayout};
use std::io::Cursor;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, Theme, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref TEXTMATE_MOCHA: Theme = {
        let mocha = include_bytes!("../assets/syntect/Catppuccin Mocha.tmTheme");
        ThemeSet::load_from_reader(&mut Cursor::new(mocha)).unwrap()
    };
    pub static ref TEXTMATE_MACCHIATO: Theme = {
        let macchiato = include_bytes!("../assets/syntect/Catppuccin Macchiato.tmTheme");
        ThemeSet::load_from_reader(&mut Cursor::new(macchiato)).unwrap()
    };
    pub static ref TEXTMATE_FRAPPE: Theme = {
        let frappe = include_bytes!("../assets/syntect/Catppuccin Frappe.tmTheme");
        ThemeSet::load_from_reader(&mut Cursor::new(frappe)).unwrap()
    };
    pub static ref TEXTMATE_LATTE: Theme = {
        let latte = include_bytes!("../assets/syntect/Catppuccin Latte.tmTheme");
        ThemeSet::load_from_reader(&mut Cursor::new(latte)).unwrap()
    };
}

pub fn highlight(
    layout: &mut TextLayout<'_, Rgba>,
    codefence: &str,
    text: &str,
    flavor: FlavorName,
) {
    let syntax = SYNTAX_SET.find_syntax_by_extension(codefence).unwrap();
    let theme = match flavor {
        FlavorName::Mocha => TEXTMATE_MOCHA.to_owned(),
        FlavorName::Macchiato => TEXTMATE_MACCHIATO.to_owned(),
        FlavorName::Frappe => TEXTMATE_FRAPPE.to_owned(),
        FlavorName::Latte => TEXTMATE_LATTE.to_owned(),
    };
    let mut h = HighlightLines::new(syntax, &theme);

    for line in LinesWithEndings::from(text) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &SYNTAX_SET).unwrap();

        for (style, text) in ranges {
            layout.push_basic_text(
                &fonts::FONT_REGULAR,
                text,
                Rgba {
                    r: style.foreground.r,
                    g: style.foreground.g,
                    b: style.foreground.b,
                    a: 255,
                },
            );
        }
    }
}
