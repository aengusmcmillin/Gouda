#![cfg(target_os = "windows")]

use cgmath::Matrix4;
use cgmath::SquareMatrix;
use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::um::d3d11::*;
use winapi::Interface;
use winapi::_core::ptr::null_mut;
use std::mem;
use winapi::shared::windef::{HWND};
use winapi::shared::winerror::FAILED;
use winapi::um::d3dcommon::*;

use crate::camera::Camera;
use crate::font_library::FontLibrary;
use crate::rendering::model::ObjModel;
use crate::rendering::shapes::ShapeLibrary;
use crate::shader_lib::ShaderLibrary;
use crate::shader_lib::imgui_shader::imgui_shader_layout;

use self::buffers::IndexBuffer;
use self::buffers::VertexBuffer;
use self::shader::Shader;
use self::texture::RenderableTexture;

pub mod buffers;
pub mod shader;
pub mod texture;

pub trait Renderable {
    fn bind(&self, scene: &Scene);
    fn num_indices(&self) -> u64;
    fn index_buffer(&self) -> &IndexBuffer;
}

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }
}

pub struct Renderer {
    swap_chain: Box<IDXGISwapChain>,
    device: *mut ID3D11Device,
    device_context: *mut ID3D11DeviceContext,
    back_buffer: Box<ID3D11Resource>,
    render_target: *mut ID3D11RenderTargetView,
    pub shader_lib: Option<ShaderLibrary>,
    pub shape_lib: Option<ShapeLibrary>,
    pub font_lib: Option<FontLibrary>,
}

impl Renderer {
    pub fn new(hwnd: HWND) -> Result<Renderer, String> {
        unsafe {
            let mut factory: *mut IDXGIFactory = null_mut();
            let result = CreateDXGIFactory(&IDXGIFactory::uuidof(), mem::transmute(&factory));
            if FAILED(result) {
                return Err("Failed to create".to_string());
            }

            let mut adapter: *mut IDXGIAdapter = null_mut();
            (*factory).EnumAdapters(0, &mut adapter);

            let mut adapter_output: *mut IDXGIOutput = null_mut();
            (*adapter).EnumOutputs(0, &mut adapter_output);

            let mut num_modes = 0;
            (*adapter_output).GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &mut num_modes, null_mut());

            let mut modes: Vec<DXGI_MODE_DESC> = Vec::new();
            for _i in 0..(num_modes) {
                modes.push(DXGI_MODE_DESC {
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL { Numerator: 0, Denominator: 0 },
                    Format: 0,
                    ScanlineOrdering: 0,
                    Scaling: 0
                });
            }
            (*adapter_output).GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &mut num_modes, modes.as_mut_ptr());

            let mut numerator = 0;
            let mut denominator = 0;
            for i in 0..(num_modes) as usize {
                if modes[i].Width == 900 {
                    if modes[i].Height == 900 {
                        numerator = modes[i].RefreshRate.Numerator;
                        denominator = modes[i].RefreshRate.Denominator;
                    }
                }
            }

            let mut adapter_desc: DXGI_ADAPTER_DESC = mem::zeroed();
            (*adapter).GetDesc(&mut adapter_desc);

            let video_memory = adapter_desc.DedicatedVideoMemory / 1024 / 1024;
            let desc = adapter_desc.Description;

            (*adapter_output).Release();
            (*adapter).Release();
            (*factory).Release();
            
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: DXGI_MODE_DESC {
                    Width: 0,
                    Height: 0,
                    RefreshRate: DXGI_RATIONAL { Numerator: 0, Denominator: 0 },
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED
                },
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 1,
                OutputWindow: hwnd,
                Windowed: 1,
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: 0
            };

            let mut swap_chain: Box<IDXGISwapChain> = Box::new(mem::zeroed());
            let mut swap_chain_ptr: *mut IDXGISwapChain = Box::into_raw(swap_chain);
            let mut device: Box<ID3D11Device> = Box::new(mem::zeroed());
            let mut device_ptr: *mut ID3D11Device = Box::into_raw(device);
            let mut device_context: Box<ID3D11DeviceContext> = Box::new(mem::zeroed());
            let mut device_context_ptr: *mut ID3D11DeviceContext = Box::into_raw(device_context);

            D3D11CreateDeviceAndSwapChain(null_mut(), D3D_DRIVER_TYPE_HARDWARE, null_mut(), D3D11_CREATE_DEVICE_DEBUG, null_mut(), 0,
            D3D11_SDK_VERSION, &swap_chain_desc, &mut swap_chain_ptr, &mut device_ptr, null_mut(), &mut device_context_ptr);

            let swap_chain = Box::from_raw(swap_chain_ptr);
            let device = Box::from_raw(device_ptr);
            let device_context = Box::from_raw(device_context_ptr);


            let mut blend_state_desc: D3D11_BLEND_DESC = mem::zeroed();
            blend_state_desc.RenderTarget[0].BlendEnable = 1;
            blend_state_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
            blend_state_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
            blend_state_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
            blend_state_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
            blend_state_desc.RenderTarget[0].RenderTargetWriteMask = 0x0f;
            let blend_state: Box<ID3D11BlendState> = Box::new(mem::zeroed());
            let mut blend_state_ptr: *mut ID3D11BlendState = Box::into_raw(blend_state);
            let result = (*device).CreateBlendState(mem::transmute(&blend_state_desc), mem::transmute(&mut blend_state_ptr));
            if FAILED(result) {
                println!("FAILED {:x}", result);
            }

            let blend_factor = [1.; 4];
            (*device_context).OMSetBlendState(blend_state_ptr, &blend_factor, 0xFFFFFFFF);

            let mut back_buffer: Box<ID3D11Resource> = Box::new(mem::zeroed());
            let mut back_buffer_ptr: *mut ID3D11Resource = Box::into_raw(back_buffer);
            (*swap_chain).GetBuffer(0, &ID3D11Resource::uuidof(), mem::transmute(&mut back_buffer_ptr));

            let mut render_target: Box<ID3D11RenderTargetView> = Box::new(mem::zeroed());
            let mut render_target_ptr: *mut ID3D11RenderTargetView = Box::into_raw(render_target);
            (*device).CreateRenderTargetView(back_buffer_ptr, null_mut(), mem::transmute(&mut render_target_ptr));

            back_buffer = Box::from_raw(back_buffer_ptr);
            render_target = Box::from_raw(render_target_ptr);

            let mut res = Renderer {
                swap_chain,
                device: Box::into_raw(device),
                device_context: Box::into_raw(device_context),
                back_buffer,
                render_target: Box::into_raw(render_target),
                shader_lib: None,
                shape_lib: None,
                font_lib: None,
            };
            let shader_lib = ShaderLibrary::construct(&res);
            res.shader_lib = Some(shader_lib);

            let shape_lib = ShapeLibrary::construct(&res);
            res.shape_lib = Some(shape_lib);

            let font_lib = FontLibrary::construct(&res);
            res.font_lib = Some(font_lib);
            return Ok(res)
        }

    }
    pub fn begin_scene(&self, camera: Box<dyn Camera>) -> Option<Scene> {
        let scene = Scene {
            device: self.device,
            device_context: self.device_context,
            render_target: self.render_target,
            swap_chain: &self.swap_chain,
            renderer: &self,
            camera_view_projection_matrix: camera.get_view_projection_matrix(),
        };

        unsafe {
            (*scene.device_context).ClearRenderTargetView(scene.render_target, &[0.43, 0.73, 0.36, 1.0]);
            (*scene.device_context).OMSetRenderTargets(1, &self.render_target, null_mut());
            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: 900.0,
                Height: 900.0,
                MinDepth: 0.0,
                MaxDepth: 1.0
            };
            (*scene.device_context).RSSetViewports(1, &viewport);
        }
        return Some(scene);
    }

    pub fn end_scene(&self, scene: Scene) {
        scene.end();
    }

    pub fn get_width(&self) -> usize {
        return 900
    }

    pub fn get_height(&self) -> usize {
        return 900
    }
}

pub struct Scene<'a> {
    pub device: *mut ID3D11Device,
    pub device_context: *mut ID3D11DeviceContext,
    pub render_target: *mut ID3D11RenderTargetView,
    pub swap_chain: &'a Box<IDXGISwapChain>,
    pub renderer: &'a Renderer,
    pub camera_view_projection_matrix: Matrix4<f32>,
}

impl Scene<'_> {
    pub fn end(self) {
        unsafe {
            let result = self.swap_chain.Present(1, 0);
            if FAILED(result) {
                panic!("Failed to present swap chain {:x}", result);
            }
        }
    }

    fn submit_impl(&self, shader: &Shader, renderable: &impl Renderable, transform: Matrix4<f32>, projection: Matrix4<f32>, color: [f32; 4]) {
        shader.bind(self);
        shader.upload_vertex_uniform_mat4(self, 0, self.camera_view_projection_matrix);
        shader.upload_vertex_uniform_mat4(self, 1, transform);
        shader.upload_fragment_uniform_float4(self, 0, color);

        renderable.bind(self);

        self.draw_indexed(renderable.num_indices(), renderable.index_buffer());
    }

    pub fn submit(&self, shader: &Shader, renderable: &impl Renderable, transform: Matrix4<f32>, color: [f32; 4]) {
        self.submit_impl(shader, renderable, transform, self.camera_view_projection_matrix, color);
    }

    pub fn submit_obj(&self, obj_model: &ObjModel, transform: Matrix4<f32>) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get("obj_model").unwrap();
        shader.bind(self);
        shader.upload_vertex_uniform_mat4(self, 0, self.camera_view_projection_matrix);
        shader.upload_vertex_uniform_mat4(self, 1, transform);

        obj_model.vertex_buffer.bind(self);

        if let Some(no_mat_ibuf) = &obj_model.no_material_index_buffer {
            no_mat_ibuf.bind(self);
            self.draw_indexed_tris(no_mat_ibuf.num_indices, &no_mat_ibuf);
        }

        obj_model.submeshes.iter().for_each(|submesh| {
            shader.upload_fragment_uniform_float3(self, 0, submesh.ambient);
            shader.upload_fragment_uniform_float3(self, 1, submesh.diffuse);
            shader.upload_fragment_uniform_float3(self, 2, [0., 0.5, -2.5]);

            submesh.index_buffer.bind(self);
            self.draw_indexed_tris(submesh.index_buffer.num_indices, &submesh.index_buffer);
        });
    }

    pub fn submit_shape_gui(&self, shader_name: &'static str, shape_name: &'static str, transform: Matrix4<f32>, color: [f32; 4]) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get(shader_name).unwrap();
        let shape = self.renderer.shape_lib.as_ref().unwrap().get(shape_name).unwrap();

        self.submit_impl(shader, shape, transform, self.camera_view_projection_matrix, color);
    }

    pub fn submit_shape_by_name(&self, shader_name: &'static str, shape_name: &'static str, transform: Matrix4<f32>, color: [f32; 4]) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get(shader_name).unwrap();
        let shape = self.renderer.shape_lib.as_ref().unwrap().get(shape_name).unwrap();

        self.submit_impl(shader, shape, transform, Matrix4::identity(), color);
    }

    fn submit_texture_with_projection(&self, texture: &RenderableTexture, transform: Matrix4<f32>, projection: Matrix4<f32>) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get("texture").unwrap();
        let shape = self.renderer.shape_lib.as_ref().unwrap().get("texture").unwrap();
        texture.bind(self);
        shader.bind(self);
        shader.upload_vertex_uniform_mat4(self, 0, projection);
        shader.upload_vertex_uniform_mat4(self, 1, transform);

        shape.bind(self);

        self.draw_indexed(shape.num_indices(), shape.index_buffer());
    }

    pub fn submit_gui_texture(&self, texture: &RenderableTexture, transform: Matrix4<f32>) {
        self.submit_texture_with_projection(texture, transform, Matrix4::identity())
    }

    pub fn submit_texture(&self, texture: &RenderableTexture, transform: Matrix4<f32>) {
        self.submit_texture_with_projection(texture, transform, self.camera_view_projection_matrix)
    }

    pub fn bind_shader(&self, shader: &'static str) {
        self.renderer.shader_lib.as_ref().unwrap().bind_shader(self, shader);
    }

    pub fn bind_font(&self, font: &'static str) {
        self.renderer.font_lib.as_ref().unwrap().get(font).unwrap().texture.bind(self);
    }

    pub fn draw_indexed(&self, num_indices: u64, index_buffer: &buffers::IndexBuffer) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            (*self.device_context).DrawIndexed(num_indices as u32, 0, 0);
        }
    }

    pub fn draw_indexed_tris(&self, num_indices: u64, index_buffer: &buffers::IndexBuffer) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            (*self.device_context).DrawIndexed(num_indices as u32, 0, 0);
        }
    }
    pub fn draw_tri_strip(&self, num_verts: u64) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            (*self.device_context).Draw(num_verts as u32, 0);
        }
    }

    pub fn draw_triangles(&self, num_verts: u64) {
        unsafe {
            (*self.device_context).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            (*self.device_context).Draw(num_verts as u32, 0);
        }
    }

    pub fn submit_imgui(&self, vbuf: &Vec<[f32; 8]>, ibuf: &[u16], count: usize, vtx_offset: usize, idx_offset: usize, texture: &RenderableTexture, matrix: Matrix4<f32>) {
        let shader = self.renderer.shader_lib.as_ref().unwrap().get("imgui").unwrap();
        shader.bind(self);
        shader.upload_vertex_uniform_mat4(self, 0, matrix);
        let vertex_buffer = VertexBuffer::new::<[f32; 8]>(self.renderer, imgui_shader_layout(), 0, vbuf.clone());
        vertex_buffer.bind(self);
        let index_buffer = IndexBuffer::new(self.renderer, ibuf.to_vec());
        index_buffer.bind_with_offset(self, idx_offset as u32);
        texture.bind(self);

        self.draw_indexed_tris(count as u64, &index_buffer);
    }
}

