use cgmath::{Matrix4, Vector3};
use imgui::{DrawData, Context, DrawCmdParams, DrawCmd};

use gouda_rendering::{Scene, texture::RenderableTexture, Renderer};
use gouda_images::Image;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawVertIntermediate {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawVert{
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [f32; 4],
}

pub struct GoudaImguiRenderer {
    font_texture: RenderableTexture
}

impl GoudaImguiRenderer {
    pub fn create(renderer: &Renderer, context: &mut Context) -> GoudaImguiRenderer {
        println!("Building imgui renderer");
        let font_texture = GoudaImguiRenderer::upload_font_texture(renderer, &mut context.fonts());

        context.set_renderer_name("gouda-imgui-renderer".to_string());

        return GoudaImguiRenderer { font_texture }
    }

    pub fn render(&self, scene: &Scene, draw_data: &DrawData) {
        for draw_list in draw_data.draw_lists() {
            let vbuf = unsafe { draw_list.transmute_vtx_buffer::<DrawVertIntermediate>() };
            let vbuf = vbuf.iter().map(|item| {
                let pos = item.pos;
                let uv = item.uv;
                let col = item.col;
                let new_color = [col[0] as f32 / 255., col[1] as f32 / 255., col[2] as f32 / 255., col[3] as f32 / 255.];
                return [pos[0], pos[1], uv[0], uv[1], new_color[0], new_color[1], new_color[2], new_color[3]];
            }).collect::<Vec<[f32; 8]>>();

            let ibuf = draw_list.idx_buffer();

            let left = draw_data.display_pos[0];
            let right = left + draw_data.display_size[0];
            let top = draw_data.display_pos[1];
            let bottom = top + draw_data.display_size[1];
            let translate = Matrix4::from_nonuniform_scale(2.0 / (right - left), 2.0 / (top - bottom), -1.);
            let scale = Matrix4::from_translation(Vector3::new((right + left) / (left - right), (top + bottom) / (bottom - top), 0.));
            let matrix = scale * translate;
            // let matrix = Matrix4::new(

            //     (2.0 / (right - left)), 
            //     0.0, 
            //     0.0,
            //     0.0,
            //     0.0, 
            //     (2.0 / (top - bottom)),
            //     0.0, 
            //     0.0,
            //     0.0, 
            //     0.0, 
            //     -1.0, 
            //     0.0,
            //     (right + left) / (left - right),
            //     (top + bottom) / (bottom - top),
            //     0.0,
            //     1.0,
            // );
            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params: 
                        DrawCmdParams {
                            vtx_offset,
                            idx_offset,
                            ..
                        }
                    } => {
                        let texture = &self.font_texture;
                        scene.submit_imgui(&vbuf, ibuf, count, vtx_offset, idx_offset, texture, matrix);
                    }
                    DrawCmd::ResetRenderState => todo!(),
                    DrawCmd::RawCallback { callback: _, raw_cmd: _ } => todo!(),
                }
            }
        }
    }
    
    fn upload_font_texture(
        renderer: &Renderer,
        fonts: &mut imgui::FontAtlas,
    ) -> RenderableTexture {
        let texture = fonts.build_rgba32_texture();
        let image = Image { width: texture.width as usize, height: texture.height as usize, data: Image::data_from_raw_pixels(texture.width as usize, texture.height as usize, texture.data.to_vec())};

        let res = RenderableTexture::new(renderer, &image, true);

        return res;
    }
}