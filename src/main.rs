use raylib::prelude::*;

mod raycasting;
use raycasting::{Player, Direction, Map, cast_rays};

const SCREEN_WIDTH:  i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;



trait UiComponent {
    fn render(&self, draw: &mut RaylibDrawHandle);
}



fn handle_keypress(key: KeyboardKey, player: &mut Player, show_map: &mut bool) {
    use KeyboardKey as K;
    match key {
        K::KEY_L | K::KEY_D | K::KEY_RIGHT => player.move_(Direction::East),
        K::KEY_H | K::KEY_A | K::KEY_LEFT  => player.move_(Direction::West),
        K::KEY_J | K::KEY_S | K::KEY_DOWN  => player.move_(Direction::South),
        K::KEY_K | K::KEY_W | K::KEY_UP    => player.move_(Direction::North),
        K::KEY_U => player.rotate(true, false),
        K::KEY_I => player.rotate(false, false),
        K::KEY_O => player.change_fov_len(true),
        K::KEY_P => player.change_fov_len(false),
        K::KEY_M => *show_map = !*show_map,
        _ => {}
    }
}

fn init_raylib() -> (RaylibHandle, RaylibThread) {

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting")
        .build();

    rl.set_target_fps(60);
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);

    (rl, thread) 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let map = Map::new();
    let mut player = Player::new();
    let mut show_map = true;

    let (mut rl, thread) = init_raylib();

    while !rl.window_should_close() {

        let key         = rl.get_key_pressed();
        let mut draw    = rl.begin_drawing(&thread);
        let mouse       = draw.get_mouse_delta();
        let mouse_wheel = draw.get_mouse_wheel_move();

        draw.clear_background(Color::from_hex("1f1f1f")?);

        //if mouse.x < 0.0 {
        //    player.rotate(true, true);
        //}
        //
        //if mouse.x > 0.0 {
        //    player.rotate(false, true);
        //}

        let ctrl = draw.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);

        if mouse_wheel > 0.0 {
            if ctrl {
                player.change_fov_width(false);
            } else {
                player.change_fov_len(false);
            }
        }

        if mouse_wheel < 0.0 {
            if ctrl {
                player.change_fov_width(true);
            } else {
                player.change_fov_len(true);
            }
        }

        if let Some(key) = key {
            handle_keypress(key, &mut player, &mut show_map);
        }

        cast_rays(&mut draw, &player, &map);

        if show_map {
            map.render(&mut draw);
            player.render(&mut draw);
        }


    }

    Ok(())
}
