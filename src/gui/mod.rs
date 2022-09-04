use cgmath::{Matrix4, Vector3};

use crate::shader_lib::gui_shader::gui_shader_layout;
use crate::types::{Color, Bounds};
use crate::rendering::{Scene, Renderer};
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, FragmentConstantBuffer, VertexConstantBuffer};
use crate::math::create_transformation_matrix;
use crate::font::{TextDrawable, Font};
use std::rc::Rc;
use crate::gui::constraints::GuiConstraints;
use crate::ecs::{ECS, Entity};
use crate::mouse_capture::{MouseCaptureArea, MouseCaptureLayer};
use crate::images::Image;
use crate::rendering::texture::RenderableTexture;

pub mod constraints;
pub mod button;
pub mod slider;

#[derive(Debug)]
pub struct ActiveGui {}

#[derive(Debug)]
pub struct GuiImage {
    drawable: RenderableTexture,
    visible: bool,
    transform: Matrix4<f32>
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
        let drawable =  RenderableTexture::new(renderer, &image);

        let transform = Matrix4::from_translation(Vector3::new(pos[0], pos[1], pos[2])) * Matrix4::from_nonuniform_scale(size[0], size[1], size[2]);

        let image = GuiImage {
            drawable,
            visible: true,
            transform
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
            scene.submit_texture(&self.drawable, self.transform);
        }
    }
}

#[derive(Debug)]
pub struct GuiText {
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
    pub calculated_bounds: Bounds,
    corner_radius: f32,
    color: Color,
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
            calculated_bounds: bounds,
            corner_radius,
            color,
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

            if let Some(layer) = ecs.write::<MouseCaptureLayer>(&layer) {
                layer.capture_areas.push(entity);
            }
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
            calculated_bounds: bounds,
            corner_radius,
            color,
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

            if let Some(layer) = ecs.write::<MouseCaptureLayer>(&layer) {
                layer.capture_areas.push(entity);
            }
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
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
    pub transform_buffer: VertexConstantBuffer,
    pub color_buffer: FragmentConstantBuffer,
    pub shape_buffer: FragmentConstantBuffer,
    pub radius_buffer: FragmentConstantBuffer,
}

impl GuiDrawable {
    pub fn new(renderer: &Renderer, radius: f32, bounds: Bounds, color: [f32; 4]) -> Self {
        let w = renderer.get_width() as f32;
        let h = renderer.get_height() as f32;
        let position = [(bounds.x as f32) / (w as f32 / 2.) - 1., (bounds.y as f32) / (h as f32 / 2.) - 1., 0.0];
        let scale = [bounds.w as f32 / w as f32, bounds.h as f32 / h as f32, 1.0];
        let radius = radius / w;

        let vb = VertexBuffer::new::<[f32; 2]>(
            renderer,
            gui_shader_layout(),
            0,
            vec![
                [0., 0.], // bottom left
                [2., 0.], // bottom right
                [2., 2.], // top right
                [0., 2.], // top left
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 3, 2,
                0, 1, 2,
            ]);


        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new::<f32>(renderer, 0, transform_mat.raw_data().to_vec());

        let color_buffer = FragmentConstantBuffer::new(renderer, 0, vec![color[0], color[1], color[2], color[3]]);
        let shape_buffer = FragmentConstantBuffer::new(renderer, 1, vec![scale[0], scale[1]]);
        let radius_buffer = FragmentConstantBuffer::new(renderer, 2, vec![radius]);

        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shape_buffer,
            radius_buffer,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        scene.bind_shader("gui".to_string());
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);
        self.color_buffer.bind(&scene);
        self.shape_buffer.bind(&scene);
        self.radius_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

