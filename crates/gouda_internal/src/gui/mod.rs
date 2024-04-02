use cgmath::{Matrix4, Vector3};
use gouda_rendering::shaders::ShaderUniform;

use crate::gui::constraints::GuiConstraints;
use crate::mouse_capture::{MouseCaptureArea, MouseCaptureLayer};
use gouda_ecs::{Entity, ECS};
use gouda_images::Image;
use gouda_rendering::font::{TextConfig, TextDrawable};
use gouda_rendering::{Renderer, Scene, Texture};
use gouda_types::{Bounds, Color};
use std::rc::Rc;

pub mod button;
pub mod constraints;
pub mod slider;

#[derive(Debug)]
pub struct ActiveGui {}

#[derive(Debug)]
pub struct GuiImage {
    drawable: Texture,
    visible: bool,
    transform: Matrix4<f32>,
}

impl GuiImage {
    pub fn create(
        ecs: &mut ECS,
        parent_bounds: Option<Bounds>,
        image: Image,
        constraints: GuiConstraints,
    ) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bounds = match parent_bounds {
            Some(parent_bounds) => constraints.calculate_bounds(parent_bounds),
            None => {
                let w = renderer.get_width() as f32;
                let h = renderer.get_height() as f32;
                constraints.calculate_bounds(Bounds { x: 0., y: 0., w, h })
            }
        };
        let w = renderer.get_width() as f32;
        let h = renderer.get_height() as f32;
        let pos = [
            (bounds.x as f32) / (w as f32 / 2.) - 1.0,
            (bounds.y as f32) / (h as f32 / 2.) - 1.0,
            1.0,
        ];
        let size = [bounds.w as f32 / w as f32, bounds.h as f32 / h as f32, 1.0];
        let drawable = Texture::new(renderer, &image);

        let transform = Matrix4::from_translation(Vector3::new(pos[0], pos[1], pos[2]))
            * Matrix4::from_nonuniform_scale(size[0], size[1], size[2]);

        let image = GuiImage {
            drawable,
            visible: true,
            transform,
        };

        ecs.build_entity().add_component(image).entity()
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn render(&self, scene: &Scene) {
        if self.visible {
            scene.submit_gui_texture(&self.drawable, self.transform);
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
    pub fn create(
        ecs: &mut ECS,
        parent_bounds: Option<Bounds>,
        text: String,
        font: &'static str,
        center_x: bool,
        center_y: bool,
        font_size: f32,
        constraints: GuiConstraints,
        color: Color,
    ) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bounds = match parent_bounds {
            Some(parent_bounds) => constraints.calculate_bounds(parent_bounds),
            None => {
                let w = renderer.get_width() as f32;
                let h = renderer.get_height() as f32;
                constraints.calculate_bounds(Bounds { x: 0., y: 0., w, h })
            }
        };
        let pos = [(bounds.x as f32) / 450. - 1., (bounds.y as f32) / 450. - 1.];
        let size = [(bounds.w as f32) / 450., (bounds.h as f32) / 450.];
        let drawable = TextDrawable::new(
            renderer,
            TextConfig {
                position: pos,
                size,
                center_x,
                center_y,
                font_name: font,
                color: [color.r, color.g, color.b],
                text,
                font_size,
            }
        );
        let text = GuiText {
            calculated_bounds: bounds,
            color,
            drawable,
            visible: true,
            center_x,
            center_y,
        };

        ecs.build_entity().add_component(text).entity()
    }

    pub fn change_text(&mut self, renderer: &Renderer, text: String, font: &'static str) {
        let bounds = self.calculated_bounds;
        let pos = [(bounds.x as f32) / 450. - 1., (bounds.y as f32) / 450. - 1.];
        let size = [(bounds.w as f32) / 450., (bounds.h as f32) / 450.];
        let drawable = TextDrawable::new(
            renderer,
            TextConfig {
                position: pos,
                size,
                center_x: self.center_x,
                center_y: self.center_y,
                font_name: font,
                color: [self.color.r, self.color.g, self.color.b],
                text,
                font_size: 16.
            }
        );
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
    pub fn create_hoverable(
        ecs: &mut ECS,
        mouse_layer: Option<Entity>,
        parent_bounds: Option<Bounds>,
        constraints: GuiConstraints,
        corner_radius: f32,
        color: Color,
        hover_color: Color,
    ) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bounds = match parent_bounds {
            Some(parent_bounds) => constraints.calculate_bounds(parent_bounds),
            None => {
                let w = renderer.get_width() as f32;
                let h = renderer.get_height() as f32;
                constraints.calculate_bounds(Bounds { x: 0., y: 0., w, h })
            }
        };
        let drawable =
            GuiDrawable::new(corner_radius, bounds, [color.r, color.g, color.b, color.a]);
        let hover_drawable = Some(GuiDrawable::new(
            corner_radius,
            bounds,
            [hover_color.r, hover_color.g, hover_color.b, hover_color.a],
        ));
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

        let mut builder = ecs.build_entity().add_component(component);
        let entity = builder.entity();
        if let Some(layer) = mouse_layer {
            builder.add_component(MouseCaptureArea::new(true, bounds));

            if let Some(layer) = ecs.write::<MouseCaptureLayer>(&layer) {
                layer.capture_areas.push(entity);
            }
        }
        entity
    }

    pub fn create(
        ecs: &mut ECS,
        mouse_layer: Option<Entity>,
        parent_bounds: Option<Bounds>,
        constraints: GuiConstraints,
        corner_radius: f32,
        color: Color,
    ) -> Entity {
        let renderer = ecs.read_res::<Rc<Renderer>>();
        let bounds = match parent_bounds {
            Some(parent_bounds) => constraints.calculate_bounds(parent_bounds),
            None => {
                let w = renderer.get_width() as f32;
                let h = renderer.get_height() as f32;
                constraints.calculate_bounds(Bounds { x: 0., y: 0., w, h })
            }
        };
        let drawable =
            GuiDrawable::new(corner_radius, bounds, [color.r, color.g, color.b, color.a]);
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
        let mut builder = ecs.build_entity().add_component(component);
        let entity = builder.entity();
        if let Some(layer) = mouse_layer {
            builder.add_component(MouseCaptureArea::new(true, bounds));

            if let Some(layer) = ecs.write::<MouseCaptureLayer>(&layer) {
                layer.capture_areas.push(entity);
            }
        }
        entity
    }

    pub fn change_color(&mut self, renderer: &Renderer, color: Color) {
        self.color = color;
        let drawable = GuiDrawable::new(
            self.corner_radius,
            self.calculated_bounds,
            [self.color.r, self.color.g, self.color.b, self.color.a],
        );
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
    pub bounds: Bounds,
    pub color: [f32; 4],
    pub radius: f32,
}

impl GuiDrawable {
    pub fn new(radius: f32, bounds: Bounds, color: [f32; 4]) -> Self {
        return Self {
            bounds,
            color,
            radius,
        };
    }

    pub fn draw(&self, scene: &Scene) {
        let w = scene.renderer.get_width() as f32;
        let h = scene.renderer.get_height() as f32;
        let position = [
            (self.bounds.x as f32) / (w as f32 / 2.) - 1.,
            (self.bounds.y as f32) / (h as f32 / 2.) - 1.,
            0.0,
        ];
        let scale = [
            self.bounds.w as f32 / w as f32,
            self.bounds.h as f32 / h as f32,
            1.0,
        ];
        let radius = self.radius / w;

        let transform_mat =
            Matrix4::from_translation(Vector3::new(position[0], position[1], position[2]))
                * Matrix4::from_nonuniform_scale(scale[0], scale[1], scale[2]);

        scene.bind_shader_with_uniforms(
            "gui",
            vec![ShaderUniform::Mat4(transform_mat)],
            vec![
                ShaderUniform::Float4(self.color),
                ShaderUniform::Float2([scale[0], scale[1]]),
                ShaderUniform::Float(radius),
            ],
        );
        scene.draw_shape("gui");
    }
}
