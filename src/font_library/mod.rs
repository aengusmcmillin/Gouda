use std::collections::HashMap;

use crate::{font::Font, rendering::Renderer};


pub struct FontLibrary {
    fonts: HashMap<&'static str, Font>
}

impl FontLibrary {
    pub fn new() -> FontLibrary {
        return FontLibrary { fonts: HashMap::new() }
    }
    pub fn construct(renderer: &Renderer) -> FontLibrary {
        let mut library = FontLibrary::new();
        let segoe_bmp = include_bytes!("segoe.png");
        let segoe_fnt = include_str!("segoe.fnt");
        library.add_font("segoe", Font::new_from_contents(renderer, segoe_bmp, segoe_fnt));

        let arial_bmp = include_bytes!("arial.png");
        let arial_fnt = include_str!("arial.fnt");
        library.add_font("arial", Font::new_from_contents(renderer, arial_bmp, arial_fnt));

        let calibri_bmp = include_bytes!("calibri.png");
        let calibri_fnt = include_str!("calibri.fnt");
        library.add_font("calibri", Font::new_from_contents(renderer, calibri_bmp, calibri_fnt));
        return library;
    }

    pub fn add_font(&mut self, name: &'static str, font: Font) {
        self.fonts.insert(name, font);
    }

}