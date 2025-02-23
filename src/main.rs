use raylib::prelude::*;

type Map = [[i32; MAP_WIDTH]; MAP_HEIGHT];

const SCREEN_WIDTH:  i32 = 1600;
const SCREEN_HEIGHT: i32 = 900;
const MAP_WIDTH:     usize = 24;
const MAP_HEIGHT:    usize = MAP_WIDTH;
const CELL_SIZE:     i32 = 35;

const OFFSET: Vector2 = Vector2::new(
    (SCREEN_WIDTH  / 2 - CELL_SIZE * MAP_WIDTH  as i32 / 2) as f32,
    (SCREEN_HEIGHT / 2 - CELL_SIZE * MAP_HEIGHT as i32 / 2) as f32
);


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




#[derive(Debug, Clone, Copy)]
struct Player {
    pub position:  Vector2,
    pub direction: Vector2,
    pub plane:     Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position:  Vector2::new(22.0, 12.0),
            direction: Vector2::new(-1.0, 0.0),
            plane:     Vector2::new(0.0, 0.66),
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {

        let pos    = self.position;
        let dir    = self.position + self.direction;
        let plane1 = dir + self.plane;
        let plane2 = dir - self.plane;

        connect_points(d, dir, plane1, Color::BLUE);  // left
        connect_points(d, dir, plane2, Color::BLUE);  // right
        connect_points(d, pos, plane2, Color::RED);   // left-diagonal
        connect_points(d, pos, plane1, Color::RED);   // right-diagonal
        connect_points(d, pos, dir,    Color::BLACK); // straight

        let point_size = 5.0;
        point(d, pos,    point_size, Color::GREEN);
        point(d, dir,    point_size, Color::BLACK);
        point(d, plane1, point_size, Color::BLUE);
        point(d, plane2, point_size, Color::BLUE);

    }

}


fn render_map(d: &mut RaylibDrawHandle, map: &Map) -> Result<(), std::num::ParseIntError> {

    let color_cell    = Color::from_hex("2d8fb5")?;
    let color_cell_bg = Color::from_hex("2e2e2e")?;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {

            let color = match *cell {
                1 => color_cell,
                2 => Color::RED,
                _ => color_cell_bg,
            };

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


fn cast_rays(d: &mut RaylibDrawHandle, player: &Player, map: &Map) {

    for x in 0..=SCREEN_WIDTH {

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;

        let ray_dir = player.direction + player.plane * camera_x;


        let mut ray = player.position;

        loop {

            ray += ray_dir.scale_by(0.1);

            //connect_points(d, ray, player.position, Color::DIMGRAY);

            if ray.x as usize >= MAP_WIDTH || ray.y as usize >= MAP_HEIGHT {
                break;
            }

            let cell = map[ray.y as usize][ray.x as usize];
            let color = match cell {
                1 => Some(Color::WHITE),
                2 => Some(Color::RED),
                _ => None,
            };

            if let Some(color) = color {
                let len = ray.length() / MAP_WIDTH as f32;
                render_stripe(d, x, CELL_SIZE, len, color);

                break;
            }

        }



    }

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let player = Player::new();

    let map: Map = [
        [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    ];

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycasting")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::from_hex("1f1f1f")?);

        render_map(&mut d, &map)?;
        player.render(&mut d);
        cast_rays(&mut d, &player, &map);

    }

    Ok(())

}
