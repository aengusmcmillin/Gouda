use gouda_images::Image;
use metal::{MTLOrigin, MTLPixelFormat, MTLRegion, MTLSize, Texture, TextureDescriptor};
use std::mem;
use std::os::raw::c_void;

use super::{PlatformRenderer, PlatformScene};

#[derive(Debug)]
pub struct PlatformTexture {
    metal_texture: Texture,
}

impl PlatformTexture {
    pub fn new(renderer: &PlatformRenderer, image: &Image) -> PlatformTexture {
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

    pub fn bind(&self, scene: &PlatformScene) {
        scene
            .encoder
            .set_fragment_texture(0, Some(&self.metal_texture));
    }
}
