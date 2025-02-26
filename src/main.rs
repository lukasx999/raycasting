use raylib::prelude::*;

mod raycasting;
use raycasting::{Player, Direction, Map, cast_rays};

const SCREEN_WIDTH:  i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;



fn handle_input(
    draw:   &mut RaylibDrawHandle,
    key:    Option<KeyboardKey>,
    player: &mut Player,
    show_minimap: &mut bool
) {

    let mouse = draw.get_mouse_delta();

    if mouse.x < 0.0 {
        player.rotate(true, true);
    }

    if mouse.x > 0.0 {
        player.rotate(false, true);
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
            K::KEY_U   => player.rotate(true, false),
            K::KEY_I   => player.rotate(false, false),
            K::KEY_O   => player.change_fov_len(true),
            K::KEY_P   => player.change_fov_len(false),
            K::KEY_TAB => *show_minimap = !*show_minimap,
            K::KEY_Q   => std::process::exit(0),
            _ => {}
        }
    }

}

fn render(
    draw:   &mut RaylibDrawHandle,
    player: &Player,
    map:    &Map,
    show_minimap: bool
) {

    draw.clear_background(Color::from_hex("1f1f1f").unwrap());

    cast_rays(draw, player, map);

    if show_minimap {
        map.render(draw);
        player.render(draw);
        draw.draw_fps(10, 10);
    }
}

fn init_raylib() -> (RaylibHandle, RaylibThread) {

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting")
        .build();

    rl.set_target_fps(60);
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);
    rl.disable_cursor();

    (rl, thread) 
}

// TODO: framebuffer for layering ui

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let map = Map::new();
    let mut player = Player::new();
    let mut show_minimap = true;

    let (mut rl, thread) = init_raylib();

    while !rl.window_should_close() {

        let key = rl.get_key_pressed(); // cannot be called after begin_drawing()
        let mut draw = rl.begin_drawing(&thread);

        handle_input(&mut draw, key, &mut player, &mut show_minimap);
        render(&mut draw, &player, &map, show_minimap);

    }

    Ok(())
}
