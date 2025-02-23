use raylib::prelude::*;



type Map = [[i32; MAP_WIDTH]; MAP_HEIGHT];

const SCREEN_WIDTH:  i32   = 1920;
const SCREEN_HEIGHT: i32   = 1080;
const MAP_WIDTH:     usize = 10;
const MAP_HEIGHT:    usize = MAP_WIDTH;
const CELL_SIZE:     i32   = 100;
const PLAYER_STEP:   f32   = 0.3;

const OFFSET: Vector2 = Vector2::new(
    (SCREEN_WIDTH  / 2 - CELL_SIZE * MAP_WIDTH  as i32 / 2) as f32,
    (SCREEN_HEIGHT / 2 - CELL_SIZE * MAP_HEIGHT as i32 / 2) as f32
);

//const OFFSET: Vector2 = Vector2::new(15.0, 15.0);


fn connect_points(d: &mut RaylibDrawHandle, p1: Vector2, p2: Vector2, color: Color) {
    let size = 3.0;
    d.draw_line_ex(
        p1 * CELL_SIZE as f32 + OFFSET,
        p2 * CELL_SIZE as f32 + OFFSET,
        size,
        color
    );
}

fn point(d: &mut RaylibDrawHandle, center: Vector2, size: f32, color: Color) {
    d.draw_circle_v(
        center * CELL_SIZE as f32 + OFFSET,
        size,
        color
    );
}



fn get_cell_color(cell: i32) -> Option<Color> {
    match cell {
        1 => Some(Color::LIGHTBLUE),
        2 => Some(Color::DARKBLUE),
        0 => None,
        _ => panic!("Unknown cell type"),
    }
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}


#[derive(Debug, Clone, Copy)]
struct Player {
    pub position:  Vector2,
    pub direction: Vector2,
    pub plane:     Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position:  Vector2::new(2.0, 5.0),
            direction: Vector2::new(1.0, 0.0),
            plane:     Vector2::new(0.0, 0.66),
        }
    }

    pub fn move_(&mut self, dir: Direction) {
        use Direction as D;
        match dir {
            D::North => self.position.y -= PLAYER_STEP,
            D::East  => self.position.x += PLAYER_STEP,
            D::South => self.position.y += PLAYER_STEP,
            D::West  => self.position.x -= PLAYER_STEP,
        }
    }

    pub fn rotate(&mut self, counter_clockwise: bool) {
        let step = if counter_clockwise { -PLAYER_STEP } else { PLAYER_STEP };
        self.direction.rotate(step);
        self.plane.rotate(step);
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let color = Color::from_hex("3888c2").unwrap();

        let pos    = self.position;
        let dir    = self.position + self.direction;
        let plane1 = dir + self.plane;
        let plane2 = dir - self.plane;

        connect_points(d, dir, plane1, color);  // left
        connect_points(d, dir, plane2, color);  // right
        connect_points(d, pos, plane2, color);  // left-diagonal
        connect_points(d, pos, plane1, color);  // right-diagonal

        point(d, pos, 10.0, color);

    }

}


fn render_map(d: &mut RaylibDrawHandle, map: &Map) -> Result<(), std::num::ParseIntError> {
    let color_cell_bg = Color::from_hex("2e2e2e")?;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = get_cell_color(*cell).unwrap_or(color_cell_bg);

            let rec = Rectangle::new(
                x as f32 * CELL_SIZE as f32 + OFFSET.x,
                y as f32 * CELL_SIZE as f32 + OFFSET.y,
                CELL_SIZE as f32,
                CELL_SIZE as f32,
            );

            d.draw_rectangle_rec(rec, color);
            d.draw_rectangle_lines_ex(rec, 1.0, color_cell_bg.brightness(0.03));

        }
    }

    Ok(())

}


fn render_stripe(
    d:     &mut RaylibDrawHandle,
    x:     i32,
    width: i32,
    size:  f32,
    color: Color
) {
    d.draw_rectangle(
        x,
        ((SCREEN_HEIGHT as f32 * size) * (1.0 - size)) as i32,
        width,
        (SCREEN_HEIGHT as f32 * size) as i32,
        color
    );
}

fn cast_rays_the_right_way(d: &mut RaylibDrawHandle, player: &Player, map: &Map) {

    let color_ray = Color::PURPLE;

    //for x in 0..=SCREEN_WIDTH {
    let x = 0; {

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
        let ray_dir = player.direction + player.plane * camera_x;

        let delta_dist = Vector2::new(
            (1.0 / ray_dir.x).abs(),
            (1.0 / ray_dir.y).abs()
        );

        // the current cell of the map
        let (mut map_x, mut map_y) = (player.position.x as usize, player.position.y as usize);

        // initial distance from player position to end of first cell
        let mut side_dist = Vector2::zero();

        let mut step = Vector2::zero();



        if ray_dir.x < 0.0 {
            step.x = -1.0;
            side_dist.x = (player.position.x - map_x as f32) * delta_dist.x;
        } else {
            step.x = 1.0;
            side_dist.x = (map_x as f32 + 1.0 - player.position.x) * delta_dist.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1.0;
            side_dist.y = (player.position.y - map_y as f32) * delta_dist.y;
        } else {
            step.y = 1.0;
            side_dist.y = (map_y as f32 + 1.0 - player.position.y) * delta_dist.y;
        }




        /* DDA */
        loop {

            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_x += step.x as usize;
            } else {
                side_dist.y += delta_dist.y;
                map_y += step.y as usize;
            }

            //if side_dist.x.abs() as usize >= MAP_WIDTH
            //|| side_dist.y.abs() as usize >= MAP_HEIGHT
            //{
            //    break;
            //}

            let cell = map[map_y][map_x];
            let color = get_cell_color(cell);

            if let Some(color) = color {
                //render_stripe(d, x, CELL_SIZE, len, color);
                break;
            }

        }


    }

}

fn cast_rays(d: &mut RaylibDrawHandle, player: &Player, map: &Map) {

    let color_ray = Color::PURPLE;

    //for x in 0..=SCREEN_WIDTH {
    let x = 0; {

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
        let ray_dir  = player.direction + player.plane * camera_x;
        let pos      = player.position;

        let delta = Vector2::new(
            (1.0 / ray_dir.x).abs(),
            (1.0 / ray_dir.y).abs()
        );

        connect_points(d, pos + ray_dir, pos, Color::RED);

        /* DDA */
        loop {


            break;

            /*
            let cell = map[map_y][map_x];
            let color = get_cell_color(cell);

            if let Some(color) = color {
                //render_stripe(d, x, CELL_SIZE, len, color);
                break;
            }
            */

        }


    }

}




fn handle_keypress(key: KeyboardKey, player: &mut Player) {
    use KeyboardKey as K;
    match key {
        K::KEY_L | K::KEY_D | K::KEY_RIGHT => player.move_(Direction::East),
        K::KEY_H | K::KEY_A | K::KEY_LEFT  => player.move_(Direction::West),
        K::KEY_J | K::KEY_S | K::KEY_DOWN  => player.move_(Direction::South),
        K::KEY_K | K::KEY_W | K::KEY_UP    => player.move_(Direction::North),
        K::KEY_U => player.rotate(true),
        K::KEY_I => player.rotate(false),
        _ => {}
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut player = Player::new();

    let map: Map = [
        [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 2, 0, 0, 0, 0, 2, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 2, 0, 0, 0, 0, 2, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
    ];

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting")
        .build();

    rl.set_target_fps(60);
    rl.set_trace_log(TraceLogLevel::LOG_ERROR);

    let mut show_map = true;

    while !rl.window_should_close() {

        let key   = rl.get_key_pressed();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::from_hex("1f1f1f")?);

        if d.is_key_pressed(KeyboardKey::KEY_M) {
            show_map = !show_map;
        }

        if let Some(key) = key {
            handle_keypress(key, &mut player);
        }

        if show_map {
            render_map(&mut d, &map)?;
            player.render(&mut d);
        }

        cast_rays(&mut d, &player, &map);

    }

    Ok(())
}
