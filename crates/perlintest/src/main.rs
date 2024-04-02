#![windows_subsystem = "windows"]

use std::{
    collections::HashMap, env
};

use gouda::{
    camera::{Camera, OrthographicCamera},
    ecs::{GameSceneId, ECS},
    rendering::{sprites::ColorBoxComponent, Scene},
    transform::TransformComponent,
    window::WindowProps,
    GameLogic, GameScene, Gouda, RenderLayer,
};

use log::trace;
use rand::seq::SliceRandom;

mod terrain;

pub const START_MENU_SCENE: GameSceneId = 0;

pub static PERMUTATIONS: [usize; 256] = [ 151,160,137,91,90,15,                 
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
];

fn fade(t: f32) -> f32 {
    6. * t.powf(5.) - 15. * t.powf(4.) + 10. * t.powf(3.)
}

fn inc(i: usize) -> usize {
    i + 1
}

fn grad(hash: usize, x: f32, y: f32, z: f32) -> f32 {
    match hash & 0xf {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 =>  x - y,
        0x3 => -x - y,
        0x4 =>  x + z,
        0x5 => -x + z,
        0x6 =>  x - z,
        0x7 => -x - z,
        0x8 =>  y + z,
        0x9 => -y + z,
        0xA =>  y - z,
        0xB => -y - z,
        0xC =>  y + x,
        0xD => -y + z,
        0xE =>  y - x,
        0xF => -y - z,
        _default => 0.
    }
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + x * (b - a)
}

fn perlin(p: [usize; 512], x: f32, y: f32) -> f32 {
    let xi = ((x as i32) & 255) as usize;
    let yi = ((y as i32) & 255) as usize;
    let zi = 0;

    let xf = x - ((x as i32) as f32);
    let yf = y - ((y as i32) as f32);
    let zf = 0.;

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = p[p[p[    xi ]+    yi ]+    zi ];
    let aba = p[p[p[    xi ]+inc(yi)]+    zi ];
    let aab = p[p[p[    xi ]+    yi ]+inc(zi)];
    let abb = p[p[p[    xi ]+inc(yi)]+inc(zi)];
    let baa = p[p[p[inc(xi)]+    yi ]+    zi ];
    let bba = p[p[p[inc(xi)]+inc(yi)]+    zi ];
    let bab = p[p[p[inc(xi)]+    yi ]+inc(zi)];
    let bbb = p[p[p[inc(xi)]+inc(yi)]+inc(zi)];

    let mut x1;
    let mut x2;
    x1 = lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1., yf, zf), u);
    x2 = lerp(grad(aba, xf, yf - 1., zf), grad(bba, xf - 1., yf - 1., zf), u);
    let y1 = lerp(x1, x2, v);
    
    x1 = lerp(grad(aab, xf, yf, zf - 1.), grad(bab, xf - 1., yf, zf - 1.), u);
    x2 = lerp(grad(abb, xf, yf - 1., zf - 1.), grad(bbb, xf - 1., yf - 1., zf - 1.), u);
    let y2 = lerp(x1, x2, v);

    (lerp(y1, y2, w) + 1.) / 2.
}

pub struct MainGameScene {}

fn shuffle(permutations: &mut [usize]) {
    let mut rng = rand::thread_rng();
    permutations.shuffle(&mut rng);
}

const RESOLUTION: i32 = 50;
impl GameScene for MainGameScene {
    fn on_scene_start(&self, ecs: &mut ECS) {
        ecs.build_entity()
            .add(Camera::Orthographic(OrthographicCamera::new(RESOLUTION as f32)))
            .add(TransformComponent::builder().build());

        let mut p: [usize; 512] = [0; 512];
        let permutations = &mut PERMUTATIONS.clone();
        shuffle(permutations);
        for i in 0..512 {
            p[i] = permutations[i % 256];
        }

        for i in (-RESOLUTION)..RESOLUTION {
            for j in (-RESOLUTION)..RESOLUTION {
                let perlin = perlin(p, ((i + RESOLUTION) as f32) / 5., ((j + RESOLUTION) as f32) / 5.);
                trace!("{}", perlin);
                ecs.build_entity()
                    .add(TransformComponent::builder()
                        .position(i as f32, j as f32)
                        .scale(0.8, 0.8)
                        .build())
                    .add(ColorBoxComponent::new([perlin, perlin, perlin]));
            }
        }
    }

    fn on_scene_stop(&self, _ecs: &mut ECS) {}

    fn render_scene(&self, ecs: &ECS, scene: &Scene) {
        let query = ecs.read2::<TransformComponent, ColorBoxComponent>();
        for (location, color_box, _) in query {
            color_box.draw(scene, location);
        }
    }

    fn next_scene(&self, _ecs: &ECS) -> Option<u32> {
        None
    }

    fn active_layers(&self, _ecs: &ECS) -> Vec<RenderLayer> {
        vec![]
    }
}
struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }
}

impl GameLogic for Game {
    fn window_props(&self) -> WindowProps {
        WindowProps {
            width: 900.0,
            height: 900.0,
            title: "Perlin Test".to_string(),
            target_ms_per_frame: 30.0,
        }
    }

    fn register_events(&self, _ecs: &mut ECS) {}

    fn migrate_events(&self, _ecs: &mut ECS) {}

    fn game_scenes(&self) -> HashMap<GameSceneId, Box<dyn GameScene>> {
        let mut res: HashMap<GameSceneId, Box<dyn GameScene>> = HashMap::new();
        res.insert(START_MENU_SCENE, Box::new(MainGameScene {}));
        res
    }

    fn initial_game_scene(&self) -> u32 {
        START_MENU_SCENE
    }

    fn setup(&mut self, _ecs: &mut ECS) {}
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let mut gouda = Gouda::new(Game::new());
    gouda.run();
}
