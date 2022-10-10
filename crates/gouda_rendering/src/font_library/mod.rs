use std::collections::HashMap;

use crate::font::Font;
use crate::platform::d3d11::PlatformRenderer;
use crate::Renderer;

pub struct FontLibrary {
    fonts: HashMap<&'static str, Font>,
}

impl FontLibrary {
    pub fn new() -> FontLibrary {
        return FontLibrary {
            fonts: HashMap::new(),
        };
    }

    pub fn initialize(&mut self, renderer: &Renderer) {
        let segoe_bmp = include_bytes!("../../../../assets/fonts/segoe.png");
        let segoe_fnt = include_str!("../../../../assets/fonts/segoe.fnt");
        self.add_font(
            "segoe",
            Font::new_from_contents(renderer, segoe_bmp, segoe_fnt),
        );

        let arial_bmp = include_bytes!("../../../../assets/fonts/arial.png");
        let arial_fnt = include_str!("../../../../assets/fonts/arial.fnt");
        self.add_font(
            "arial",
            Font::new_from_contents(renderer, arial_bmp, arial_fnt),
        );

        let calibri_bmp = include_bytes!("../../../../assets/fonts/calibri.png");
        let calibri_fnt = include_str!("../../../../assets/fonts/calibri.fnt");
        self.add_font(
            "calibri",
            Font::new_from_contents(renderer, calibri_bmp, calibri_fnt),
        );
    }

    pub fn add_font(&mut self, name: &'static str, font: Font) {
        self.fonts.insert(name, font);
    }

    pub fn get(&self, name: &'static str) -> Option<&Font> {
        return self.fonts.get(name);
    }
}
