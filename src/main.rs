use std::sync::{Arc, Mutex};

use raylib::prelude::*;

mod map;
use map::{Map, MAP_CELL_SIZE, MAP_WIDTH, MAP_HEIGHT};

mod player;
use player::{Player, Direction};

mod raycasting;
use raycasting::{Raycaster, OFFSET};

const SCREEN_WIDTH:  i32 = 1600;
const SCREEN_HEIGHT: i32 = 900;

//const SCREEN_WIDTH:  i32 = 640;
//const SCREEN_HEIGHT: i32 = 480;


type Stripe = Arc<Mutex<[Color; SCREEN_HEIGHT as usize]>>;

#[derive(Debug, Clone)]
struct Framebuffer(Box<[Stripe; SCREEN_WIDTH as usize]>);

impl Framebuffer {

    pub fn new(color: Color) -> Self {
        use std::array::from_fn as array;

        // using this function, because Arc doesnt implement Copy
        let fb = array::<_, { SCREEN_WIDTH as usize }, _>(|_|
            Arc::new(Mutex::new([color; SCREEN_HEIGHT as usize]))
        );

        Self(Box::new(fb))

    }

    pub fn clear(&mut self, color: Color) {
        for stripe in self.0.iter_mut() {
            for c in stripe.lock().unwrap().iter_mut() {
                *c = color;
            }
        }
    }

    pub fn render(&self, draw: &mut impl RaylibDraw) {
        for (y, stripe) in self.0.iter().enumerate() {
            for (x, color) in stripe.lock().unwrap().iter().enumerate() {
                draw.draw_rectangle(x as i32, y as i32, 1, 1, color);
            }
        }
    }
}



type TextureDrawHandle<'a> = RaylibTextureMode<'a, RaylibDrawHandle<'a>>;



struct Application {
    raycaster:    Raycaster,
    player:       Player,
    map:          Map,
    show_minimap: bool,
}

impl Application {

    pub fn new() -> Self {
        Self {
            raycaster: Raycaster::new(),
            player: Player::new(),
            map: Map::new(),
            show_minimap: false
        }
    }

    pub fn render(
        &mut self,
        fb:              &mut Framebuffer,
        thread:          &RaylibThread,
        draw:            &mut RaylibDrawHandle,
        texture_minimap: &mut RenderTexture2D
    ) {

        let Self { raycaster, player, map, show_minimap } = self;

        draw.clear_background(Color::from_hex("1f1f1f").unwrap());

        {
            let mut texture_draw = draw.begin_texture_mode(&thread, texture_minimap);
            map.render(&mut texture_draw);
        }

        raycaster.cast_rays(fb, player, map);
        fb.clear(Color::BLACK);

        // Draw the players FOV above the rays from render_world_3d()
        {
            let mut texture_draw = draw.begin_texture_mode(&thread, texture_minimap);
            player.render(&mut texture_draw);
        }

        if *show_minimap {

            // texture has to be y-flipped because of some OpenGL BS
            let rec = Rectangle::new(
                0.0,
                0.0,
                texture_minimap.width() as f32,
                -texture_minimap.height() as f32
            );

            draw.draw_texture_rec(&texture_minimap, rec, OFFSET, Color::WHITE);
        }

        draw.draw_fps(10, 10);

    }

    pub fn handle_input(&mut self, draw: &mut RaylibDrawHandle, key: Option<KeyboardKey>) {
        let Self { player, show_minimap, .. } = self;

        let mouse = draw.get_mouse_delta();
        let ctrl  = draw.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);

        if mouse.x < 0.0 || draw.is_key_down(KeyboardKey::KEY_H) {
            player.rotate(true);
        }

        if mouse.x > 0.0 || draw.is_key_down(KeyboardKey::KEY_L) {
            player.rotate(false);
        }

        if draw.is_key_down(KeyboardKey::KEY_D) {
            player.move_(Direction::East);
        }

        if draw.is_key_down(KeyboardKey::KEY_A) {
            player.move_(Direction::West);
        }

        if draw.is_key_down(KeyboardKey::KEY_S) {
            player.move_(Direction::South);
        }

        if draw.is_key_down(KeyboardKey::KEY_W) {
            player.move_(Direction::North);
        }

        if let Some(key) = key {
            use KeyboardKey as K;
            match key {
                K::KEY_J if ctrl => player.increase_fov_width(true),
                K::KEY_K if ctrl => player.increase_fov_width(false),
                K::KEY_J => player.increase_fov(true),
                K::KEY_K => player.increase_fov(false),
                K::KEY_TAB => *show_minimap = !*show_minimap,
                K::KEY_Q   => std::process::exit(0),
                _ => {}
            }
        }

    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut app = Application::new();

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .log_level(TraceLogLevel::LOG_ERROR)
        .title("Raycasting")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor();


    let mut texture_minimap = rl.load_render_texture(
        &thread,
        MAP_WIDTH  as u32 * MAP_CELL_SIZE as u32,
        MAP_HEIGHT as u32 * MAP_CELL_SIZE as u32,
    )?;

    let mut fb = Framebuffer::new(Color::BLACK);

    while !rl.window_should_close() {

        let key = rl.get_key_pressed(); // cannot be called after begin_drawing()
        let mut draw = rl.begin_drawing(&thread);

        app.handle_input(&mut draw, key);
        app.render(&mut fb, &thread, &mut draw, &mut texture_minimap);

    }

    Ok(())
}
