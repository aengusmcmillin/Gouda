pub struct Renderer {
    renderer_impl: Box<dyn PlatformRendererImpl>,
}

impl Renderer {
    pub fn new(renderer_impl: Box<dyn PlatformRendererImpl>) -> Self {
        Self {
            renderer_impl,
        }
    }

    pub fn render(&self) {
        self.renderer_impl.render();
    }
}

pub trait PlatformRendererImpl {
    fn render(&self);
}

