use metal::{Texture, TextureDescriptor, MTLPixelFormat, MTLRegion, MTLOrigin, MTLSize};
use crate::platform::metal::{Renderer, Scene};
use crate::bmp::Bitmap;
use std::mem;
use std::os::raw::c_void;
use crate::png::PNG;

#[derive(Debug)]
pub struct RenderableTexture {
    metal_texture: Texture,
}

impl RenderableTexture {
    pub fn new_from_png(renderer: &Renderer, png: PNG) -> RenderableTexture {
        let width = png.header_chunk.width as u64;
        let height = png.header_chunk.height as u64;

        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        descriptor.set_width(width);
        descriptor.set_height(height);
        let metal_texture = renderer.device.new_texture(&descriptor);

        let region = MTLRegion {
            origin: MTLOrigin {x: 0, y: 0, z: 0},
            size: MTLSize {width, height, depth: 1 }
        };
        let data: *const c_void = unsafe {
            mem::transmute(png.data.as_ptr())
        };
        metal_texture.replace_region(region, 0, 4 * width, data);

        let result = Self {
            metal_texture,
        };
        return result;
    }

    pub fn new(renderer: &Renderer, bitmap: Bitmap) -> RenderableTexture {
        let width = bitmap.header.width as u64;
        let height = bitmap.header.height as u64;

        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        descriptor.set_width(width);
        descriptor.set_height(height);
        let metal_texture = renderer.device.new_texture(&descriptor);

        let region = MTLRegion {
            origin: MTLOrigin {x: 0, y: 0, z: 0},
            size: MTLSize {width, height, depth: 1 }
        };
        let data: *const c_void = unsafe {
            mem::transmute(bitmap.contents.as_ptr())
        };
        metal_texture.replace_region(region, 0, 4 * width, data);

        let result = Self {
            metal_texture,
        };
        return result;
    }

    pub fn bind(&self, scene: &Scene) {
        scene.encoder.set_fragment_texture(0, Some(&self.metal_texture));
    }
}