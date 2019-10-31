use crate::types::{Color, Bounds};
use crate::rendering::{Scene, Renderer};
use crate::rendering::buffers::{VertexBuffer, IndexBuffer, FragmentConstantBuffer, VertexConstantBuffer};
use crate::rendering::shader::Shader;
use crate::math::{Mat4x4, create_transformation_matrix};

#[derive(Debug)]
pub struct ActiveGui {}

#[derive(Debug)]
pub struct GuiComponent {
    constraints: GuiConstraints,
    calculated_bounds: Bounds,
    corner_radius: f32,
    color: Color,
    children: Vec<GuiComponent>,
    drawable: GuiDrawable,
}

impl GuiComponent {
    pub fn new(renderer: &Renderer, parent: Option<&GuiComponent>, constraints: GuiConstraints, corner_radius: f32, color: Color) -> GuiComponent {
        let bounds = match parent {
            Some(parent) => {
                constraints.calculate_bounds(parent.calculated_bounds)
            }
            None => {
                constraints.calculate_bounds(Bounds {x: 0, y: 0, w: 900, h: 900})
            }
        };
        let drawable = GuiDrawable::new(renderer, [color.r, color.g, color.b], [(bounds.x as f32) / 450. - 1., (bounds.y as f32) / 450. - 1., 1.0], [bounds.w as f32 / 900., bounds.h as f32 / 900., 1.0]);
        GuiComponent {
            constraints,
            calculated_bounds: bounds,
            corner_radius,
            color,
            children: Vec::new(),
            drawable,
        }
    }

    pub fn add_child(&mut self, component: GuiComponent) {
        self.children.push(component);
    }

    pub fn render(&self, scene: &Scene) {
        self.drawable.draw(scene);

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
    pub identity_buffer: VertexConstantBuffer<f32>,
}

impl GuiDrawable {
    pub fn new(renderer: &Renderer, color: [f32; 3], position: [f32; 3], scale: [f32; 3]) -> Self {
        let vb = VertexBuffer::new(
            renderer,
            0,
            vec![
                0., 0., 0., 1.,
                2., 0., 0., 1.,
                2., 2., 0., 1.,
                0., 2., 0., 1.,
            ]);

        let ib = IndexBuffer::new(
            renderer,
            vec![
                0, 1, 2, 2, 3, 0
            ]);

        let shader = Shader::new(
            renderer,
            false,
            "shaders/guiVertexShader.txt",
            "shaders/guiFragmentShader.txt");

        let transform_mat = create_transformation_matrix(position, [0., 0., 0.], scale);
        let transform_buffer = VertexConstantBuffer::new(renderer,0, transform_mat.raw_data().to_vec());

        let color_buffer = FragmentConstantBuffer::new(renderer, 0, vec![color[0], color[1], color[2], 1.0]);

        let identity_buffer = VertexConstantBuffer::new(renderer, 1, Mat4x4::identity().to_vec());
        return Self {
            vertex_buffer: vb,
            index_buffer: ib,
            transform_buffer,
            color_buffer,
            shader,
            identity_buffer,
        }
    }

    pub fn draw(&self, scene: &Scene) {
        self.shader.bind(scene);
        self.vertex_buffer.bind(scene);
        self.transform_buffer.bind(scene);

        self.color_buffer.bind(&scene);

        scene.draw_indexed(6, &self.index_buffer);
    }
}

#[derive(Debug)]
pub struct GuiConstraints {
    x: Constraint,
    y: Constraint,
    width: Constraint,
    height: Constraint,
}

impl GuiConstraints {
    pub fn new(x: Constraint, y: Constraint, width: Constraint, height: Constraint) -> GuiConstraints {
        GuiConstraints {
            x,
            y,
            width,
            height
        }
    }

    pub fn calculate_bounds(&self, parent: Bounds) -> Bounds {
        let mut x = self.x.calculate(parent.x, parent.x + parent.w);
        let mut y = self.y.calculate(parent.y, parent.y + parent.h);
        let mut w = self.width.calculate(0, parent.w);
        let mut h = self.height.calculate(0, parent.h);

        if w == 0 && h == 0 {
            panic!("Bad width and height");
        } else if let Constraint::AspectConstraint {aspect: aspect} = self.width {
            w = (h as f32 * aspect) as i32;
        } else if let Constraint::AspectConstraint {aspect: aspect} = self.height {
            h = (w as f32 * aspect) as i32;
        }

        if let Constraint::CenterConstraint = self.x {
            x -= (w as f32 * 0.5) as i32;
        }
        if let Constraint::CenterConstraint = self.y {
            y -= (h as f32 * 0.5) as i32;
        }

        println!("{} {} {} {}", x, y, w, h);
        return Bounds {
            x,
            y,
            w,
            h
        }
    }
}

#[derive(Debug)]
pub enum Constraint {
    RelativeConstraint{size: f32},
    CenterConstraint,
    PixelConstraint{size: i32},
    AspectConstraint{aspect: f32},
}

impl Constraint {
    pub fn calculate(&self, min: i32, max: i32) -> i32 {
        match self {
            Constraint::RelativeConstraint { size: size } => {
                min + ((max - min) as f32 * size) as i32
            }
            Constraint::CenterConstraint => {
                (min + max) / 2
            }
            Constraint::PixelConstraint { size: size } => {
                if *size < 0 {
                    max - min + *size
                } else {
                    *size
                }
            }
            Constraint::AspectConstraint { aspect: _ } => {
                0
            }
        }
    }
}
