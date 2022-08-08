use crate::shader_lib::gui_shader::{GUI_VERTEX_SHADER, GUI_FRAGMENT_SHADER};
use crate::types::{Color, Bounds};
use crate::rendering::{Scene, Renderer};
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, FragmentConstantBuffer, VertexConstantBuffer, ShaderDataType, BufferLayout, BufferElement};
use crate::rendering::shader::Shader;
use crate::math::{Mat4x4, create_transformation_matrix};
use crate::rendering::drawable::TextureDrawable;
use crate::font::{TextDrawable, Font};
use std::rc::Rc;
use crate::gui::constraints::GuiConstraints;
use crate::ecs::{ECS, Entity};
use crate::mouse_capture::{MouseCaptureArea, MouseCaptureLayer};
use crate::images::Image;
use crate::platform::metal::texture::RenderableTexture;

pub mod constraints;
pub mod button;
pub mod slider;

#[derive(Debug)]
pub struct ActiveGui {}

#[derive(Debug)]
pub struct GuiImage {
    constraints: GuiConstraints,
    calculated_bounds: Bounds,
    drawable: TextureDrawable,
    visible: bool,
}

impl GuiImage {

    pub fn create(ecs: &mut ECS, parent_bounds: Option<Bounds>, image: Image, constraints: GuiConstraints) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
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
        let w = renderer.get_width() as f32;
        let h = renderer.get_height() as f32;
        let pos = [(bounds.x as f32) / (w as f32 / 2.) - 1.0, (bounds.y as f32) / (h as f32 / 2.) - 1.0, 1.0];
        let size = [bounds.w as f32 / w as f32, bounds.h as f32 / h as f32, 1.0];
        let renderable_texture=  RenderableTexture::new(renderer, &image);

        let mut drawable = TextureDrawable::new(true, renderer, renderable_texture);
        drawable.set_position(pos);
        drawable.set_scale(size);

        let image = GuiImage {
            constraints,
            calculated_bounds: bounds,
            drawable,
            visible: true,
        };

        ecs.build_entity().add(image).entity()
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn render(&self, scene: &Scene) {
        if self.visible {
            self.drawable.draw(scene);
        }
    }
}

#[derive(Debug)]
pub struct GuiText {
    constraints: GuiConstraints,
    calculated_bounds: Bounds,
    color: Color,
    drawable: TextDrawable,
    visible: bool,
    center_x: bool,
    center_y: bool,
}

impl GuiText {
    pub fn create(ecs: &mut ECS, parent_bounds: Option<Bounds>, text: String, font: Rc<Font>, center_x: bool, center_y: bool, font_size: f32, constraints: GuiConstraints, color: Color) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
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
        let pos = [(bounds.x as f32) / 450. - 1., (bounds.y as f32) / 450. - 1.];
        let size = [(bounds.w as f32) / 450., (bounds.h as f32) / 450.];
        let drawable = TextDrawable::new(renderer,  pos, size, center_x, center_y, font, [color.r, color.g, color.b], text, font_size);
        let text = GuiText {
            constraints,
            calculated_bounds: bounds,
            color,
            drawable,
            visible: true,
            center_x,
            center_y
        };

        ecs.build_entity().add(text).entity()
    }

    pub fn change_text(&mut self, renderer: &Renderer, text: String, font: Rc<Font>) {
        let bounds = self.calculated_bounds;
        let pos = [(bounds.x as f32) / 450. - 1., (bounds.y as f32) / 450. - 1.];
        let size = [(bounds.w as f32) / 450., (bounds.h as f32) / 450.];
        let drawable = TextDrawable::new(renderer, pos, size, self.center_x, self.center_y, font, [self.color.r, self.color.g, self.color.b], text, 16.);
        self.drawable = drawable;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn render(&self, scene: &Scene) {
        if self.visible {
            self.drawable.draw(scene);
        }
    }
}

#[derive(Debug)]
pub struct GuiComponent {
    constraints: GuiConstraints,
    pub calculated_bounds: Bounds,
    corner_radius: f32,
    color: Color,
    hover_color: Option<Color>,
    children: Vec<Entity>,
    text: Vec<Entity>,
    images: Vec<Entity>,
    drawable: GuiDrawable,
    hover_drawable: Option<GuiDrawable>,
    is_hovered: bool,
    visible: bool,
}

impl GuiComponent {
    pub fn create_hoverable(ecs: &mut ECS, mouse_layer: Option<Entity>, parent_bounds: Option<Bounds>, constraints: GuiConstraints, corner_radius: f32, color: Color, hover_color: Color) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
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
        let drawable = GuiDrawable::new(renderer, corner_radius, bounds, [color.r, color.g, color.b, color.a]);
        let hover_drawable = Some(GuiDrawable::new(renderer, corner_radius, bounds, [hover_color.r, hover_color.g, hover_color.b, hover_color.a]));
        let component = GuiComponent {
            constraints,
            calculated_bounds: bounds,
            corner_radius,
            color,
            hover_color: Some(hover_color),
            text: Vec::new(),
            images: Vec::new(),
            children: Vec::new(),
            drawable,
            hover_drawable,
            is_hovered: false,
            visible: true,
        };

        let mut builder = ecs.build_entity().add(component);
        let entity = builder.entity();
        if let Some(layer) = mouse_layer {
            builder.add(MouseCaptureArea::new(bounds));

            let layer = ecs.write::<MouseCaptureLayer>(&layer).unwrap();
            layer.capture_areas.push(entity);
        }
        entity
    }

    pub fn create(ecs: &mut ECS, mouse_layer: Option<Entity>, parent_bounds: Option<Bounds>, constraints: GuiConstraints, corner_radius: f32, color: Color) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
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
        let drawable = GuiDrawable::new(renderer, corner_radius, bounds, [color.r, color.g, color.b, color.a]);
        let component = GuiComponent {
            constraints,
            calculated_bounds: bounds,
            corner_radius,
            color,
            hover_color: None,
            text: Vec::new(),
            images: Vec::new(),
            children: Vec::new(),
            drawable,
            hover_drawable: None,
            is_hovered: false,
            visible: true,
        };
        let mut builder = ecs.build_entity().add(component);
        let entity = builder.entity();
        if let Some(layer) = mouse_layer {
            builder.add(MouseCaptureArea::new(bounds));

            let layer = ecs.write::<MouseCaptureLayer>(&layer).unwrap();
            layer.capture_areas.push(entity);
        }
        entity
    }

    pub fn change_color(&mut self, renderer: &Renderer, color: Color) {
        self.color = color;
        let drawable = GuiDrawable::new(renderer, self.corner_radius, self.calculated_bounds, [self.color.r, self.color.g, self.color.b, self.color.a]);
        self.drawable = drawable;
    }

    pub fn add_child(&mut self, component: Entity) {
        self.children.push(component);
    }

    pub fn add_text(&mut self, text: Entity) -> &mut Self {
        self.text.push(text);
        self
    }

    pub fn add_image(&mut self, image: Entity) {
        self.images.push(image);
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn render(&self, ecs: &ECS, scene: &Scene) {
        if !self.visible {
            return;
        }

        if self.is_hovered {
            if let Some(hover_drawable) = &self.hover_drawable {
                hover_drawable.draw(scene);
            } else {
                self.drawable.draw(scene);
            }
        } else {
            self.drawable.draw(scene);
        }

        for text in &self.text {
            ecs.read::<GuiText>(text).unwrap().render(scene);
        }

        for image in &self.images {
            ecs.read::<GuiImage>(image).unwrap().render(scene);
        }

        for child in &self.children {
            ecs.read::<GuiComponent>(child).unwrap().render(&ecs, scene);
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
    pub fn new(renderer: &Renderer, radius: f32, bounds: Bounds, color: [f32; 4]) -> Self {
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

        let buffer_layout = BufferLayout::new(
            vec![
                BufferElement::new("position".to_string(), ShaderDataType::Float4),
                BufferElement::new("texCoord".to_string(), ShaderDataType::Float2)
            ]
        );
        let shader = Shader::new(
            renderer,
            buffer_layout,
            GUI_VERTEX_SHADER,
            GUI_FRAGMENT_SHADER);

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let color_buffer = FragmentConstantBuffer::new(renderer, 0, vec![color[0], color[1], color[2], color[3]]);
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

