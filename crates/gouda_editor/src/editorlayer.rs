use std::rc::Rc;
use std::time::Duration;

use gouda::ecs::ECS;
use gouda::imgui::platform::*;
use gouda::imgui::renderer::*;
use gouda::imgui::ConfigFlags;
use gouda::imgui::Context;
use gouda::imgui::FontConfig;
use gouda::imgui::Ui;
use gouda::input::GameInput;
use gouda::rendering::Renderer;
use gouda::rendering::Scene;
use gouda::layer::Layer;
use gouda::imgui::FontSource;

pub struct EditorLayer {
    imgui: Option<Context>,
    imgui_platform: Option<GoudaImguiPlatform>,
    imgui_renderer: Option<GoudaImguiRenderer>,
}

impl EditorLayer {
    pub fn new() -> EditorLayer {
        return EditorLayer {
            imgui: None,
            imgui_platform: None,
            imgui_renderer: None,
        };
    }
}

impl Layer for EditorLayer {
    fn update(&mut self, ecs: &ECS, dt: f32) {
        let input = ecs.read_res::<GameInput>();
        if let Some(imgui) = self.imgui.as_mut() {
            GoudaImguiPlatform::prepare_frame(imgui, Duration::from_secs_f32(dt));

            {
                let io = imgui.io_mut();
                io.mouse_pos = [input.mouse.x as f32, input.mouse.y as f32];
                io.mouse_down[0] = input.mouse.buttons[0].ended_down;
            }

            let mut opened: bool = false;
            let ui: &mut Ui = imgui.new_frame();

            if let Some(menu_bar) = ui.begin_main_menu_bar() {
                if let Some(menu) = ui.begin_menu("File") {
                    ui.menu_item("Save");
                    menu.end()
                }
                menu_bar.end()
            }
        }
    }

    fn render(&mut self, ecs: &mut ECS, scene: &mut Scene) {
        if let Some(imgui) = self.imgui.as_mut() {
            let draw_data = imgui.render();
            if let Some(imgui_renderer) = self.imgui_renderer.as_ref() {
                imgui_renderer.render(&scene, draw_data);
            }
        }
    }

    fn setup(&mut self, gouda: &ECS) {
        let renderer = gouda.read_res::<Rc<Renderer>>();
        let mut imgui = Context::create();
        imgui.set_ini_filename(None);
        imgui.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE;
        imgui.io_mut().config_flags |= ConfigFlags::VIEWPORTS_ENABLE;

        GoudaImguiPlatform::init(&mut imgui);

        imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../../../assets/Roboto-Regular.ttf"),
            size_pixels: 13.0,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        }]);

        self.imgui_renderer = Some(GoudaImguiRenderer::create(&renderer, &mut imgui));
        self.imgui = Some(imgui);
    }
}
