use crate::types::{Color, Bounds};
use crate::rendering::{Scene, Renderer};
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, FragmentConstantBuffer, VertexConstantBuffer};
use crate::rendering::shader::Shader;
use crate::math::{Mat4x4, create_transformation_matrix};
use crate::rendering::drawable::QuadDrawable;
use crate::font::{TextDrawable, Font};
use std::rc::Rc;
use crate::gui::constraints::GuiConstraints;

pub mod constraints;
pub mod button;
pub mod slider;

#[derive(Debug)]
pub struct ActiveGui {}

#[derive(Debug)]
pub struct GuiText {
    constraints: GuiConstraints,
    calculated_bounds: Bounds,
    color: Color,
    drawable: TextDrawable,
}

impl GuiText {
    pub fn new(renderer: &Renderer, parent_bounds: Option<Bounds>, text: String, font: Rc<Font>, constraints: GuiConstraints, color: Color) -> GuiText {
        let bounds = match parent_bounds {
            Some(parent) => {
                constraints.calculate_bounds(parent)
            }
            None => {
                constraints.calculate_bounds(Bounds {x: 0, y: 0, w: 900, h: 900})
            }
        };
        let pos = [(bounds.x as f32) / 450., (bounds.y as f32) / 450.];
        let drawable = TextDrawable::new(renderer, pos, font, text, 12.);
        GuiText {
            constraints,
            calculated_bounds: bounds,
            color,
            drawable,
        }
    }

    pub fn render(&self, scene: &Scene) {
        self.drawable.draw(scene);
    }
}

#[derive(Debug)]
pub struct GuiComponent {
    constraints: GuiConstraints,
    pub calculated_bounds: Bounds,
    corner_radius: f32,
    color: Color,
    children: Vec<GuiComponent>,
    text: Vec<GuiText>,
    drawable: GuiDrawable,
}

impl GuiComponent {
    pub fn new(renderer: &Renderer, parent_bounds: Option<Bounds>, constraints: GuiConstraints, corner_radius: f32, color: Color) -> GuiComponent {
        let bounds = match parent_bounds {
            Some(parent_bounds) => {
                constraints.calculate_bounds(parent_bounds)
            }
            None => {
                let w = renderer.get_width() as i32;
                let h = renderer.get_height() as i32;
                constraints.calculate_bounds(Bounds {x: 0, y: 0, w, h})
            }
        };
        let drawable = GuiDrawable::new(renderer, corner_radius, bounds, [color.r, color.g, color.b]);
        GuiComponent {
            constraints,
            calculated_bounds: bounds,
            corner_radius,
            color,
            text: Vec::new(),
            children: Vec::new(),
            drawable,
        }
    }

    pub fn recalculate(&mut self, renderer: &Renderer, parent_bounds: Option<Bounds>) {
        let bounds = match parent_bounds {
            Some(parent_bounds) => {
                self.constraints.calculate_bounds(parent_bounds)
            }
            None => {
                let w = renderer.get_width() as i32;
                let h = renderer.get_height() as i32;
                self.constraints.calculate_bounds(Bounds {x: 0, y: 0, w, h})
            }
        };
        let drawable = GuiDrawable::new(renderer, self.corner_radius, bounds, [self.color.r, self.color.g, self.color.b]);
        self.drawable = drawable;
        self.calculated_bounds = bounds;

        self.children.iter_mut().map(|child| child.recalculate(renderer, Some(bounds)));
    }

    pub fn change_color(&mut self, renderer: &Renderer, color: Color) {
        self.color = color;
        let drawable = GuiDrawable::new(renderer, self.corner_radius, self.calculated_bounds, [self.color.r, self.color.g, self.color.b]);
        self.drawable = drawable;
    }

    pub fn add_child(&mut self, component: GuiComponent) {
        self.children.push(component);
    }

    pub fn add_text(&mut self, text: GuiText) {
        self.text.push(text);
    }

    pub fn render(&self, scene: &Scene) {
        self.drawable.draw(scene);

        for text in &self.text {
            text.render(scene);
        }

        for child in &self.children {
            child.render(scene);
        }
    }
}

#[derive(Debug)]
pub struct GuiDrawable {
    pub vertex_buffer: VertexBuffer<f32>,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer<f32>,
    pub shader: Shader,
    pub color_buffer: FragmentConstantBuffer<f32>,
    pub shape_buffer: FragmentConstantBuffer<f32>,
    pub radius_buffer: FragmentConstantBuffer<f32>,
    pub identity_buffer: VertexConstantBuffer<f32>,
}

impl GuiDrawable {
    pub fn new(renderer: &Renderer, radius: f32, bounds: Bounds, color: [f32; 3]) -> Self {
        let w = renderer.get_width() as f32;
        let h = renderer.get_height() as f32;
        let position = [(bounds.x as f32) / (w as f32 / 2.) - 1.0, (bounds.y as f32) / (h as f32 / 2.) - 1.0, 1.0];
        let scale = [bounds.w as f32 / w as f32, bounds.h as f32 / h as f32, 1.0];
        let radius = radius / w;

        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                0., 0., 0., 1., 0., 1.,
                2., 0., 0., 1., 1., 1.,
                2., 2., 0., 1., 1., 0.,
                0., 2., 0., 1., 0., 0.,
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 3, 2,
                0, 1, 2,
            ]);

        let shader = Shader::new(
            renderer,
            true,
            "shaders/guiVertexShader.txt",
            "shaders/guiFragmentShader.txt");

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let color_buffer = FragmentConstantBuffer::new(renderer, 0, vec![color[0], color[1], color[2], 0.5]);
        let shape_buffer = FragmentConstantBuffer::new(renderer, 1, vec![scale[0], scale[1]]);
        let radius_buffer = FragmentConstantBuffer::new(renderer, 2, vec![radius]);

        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());
        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shape_buffer,
            radius_buffer,
            shader,
            identity_buffer,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.identity_buffer.bind(scene);
        self.index_buffer.bind(scene);
        self.color_buffer.bind(&scene);
        self.shape_buffer.bind(&scene);
        self.radius_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

