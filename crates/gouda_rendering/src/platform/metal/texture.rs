use crate::images::Image;
use crate::platform::metal::{Renderer, Scene};
use metal::{MTLOrigin, MTLPixelFormat, MTLRegion, MTLSize, Texture, TextureDescriptor};
use std::mem;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct RenderableTexture {
    metal_texture: Texture,
}

impl RenderableTexture {
    pub fn new(renderer: &Renderer, image: &Image) -> RenderableTexture {
        let width = image.width as u64;
        let height = image.height as u64;

        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        descriptor.set_width(width);
        descriptor.set_height(height);
        let metal_texture = renderer.device.new_texture(&descriptor);

        let region = MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width,
                height,
                depth: 1,
            },
        };
        let data: *const c_void = unsafe { mem::transmute(image.raw_pixels().as_ptr()) };
        metal_texture.replace_region(region, 0, 4 * width, data);

        let result = Self { metal_texture };
        return result;
    }

    pub fn bind(&self, scene: &Scene) {
        scene
            .encoder
            .set_fragment_texture(0, Some(&self.metal_texture));
    }
}
