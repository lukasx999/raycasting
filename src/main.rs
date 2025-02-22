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

        let circle_size = 3.0;

        let pos = self.position;
        let dir = self.position + self.direction;
        let plane_right = dir + self.plane;
        let plane_left  = dir - self.plane;

        d.draw_circle_v(
            pos * CELL_SIZE as f32 + OFFSET,
            circle_size,
            Color::GREEN
        );

        d.draw_circle_v(
            dir * CELL_SIZE as f32 + OFFSET,
            circle_size,
            Color::BLACK
        );

        d.draw_line_v(
            pos * CELL_SIZE as f32 + OFFSET,
            dir * CELL_SIZE as f32 + OFFSET,
            Color::BLACK
        );

        d.draw_circle_v(
            plane_right * CELL_SIZE as f32 + OFFSET,
            circle_size,
            Color::BLUE
        );

        d.draw_circle_v(
            plane_left * CELL_SIZE as f32 + OFFSET,
            circle_size,
            Color::BLUE
        );

        d.draw_line_v(
            dir * CELL_SIZE as f32 + OFFSET,
            plane_right * CELL_SIZE as f32 + OFFSET,
            Color::BLUE
        );

        d.draw_line_v(
            dir * CELL_SIZE as f32 + OFFSET,
            plane_left * CELL_SIZE as f32 + OFFSET,
            Color::BLUE
        );

        d.draw_line_v(
            pos * CELL_SIZE as f32 + OFFSET,
            plane_left * CELL_SIZE as f32 + OFFSET,
            Color::RED
        );

        d.draw_line_v(
            pos * CELL_SIZE as f32 + OFFSET,
            plane_right * CELL_SIZE as f32 + OFFSET,
            Color::RED
        );




    }

}


fn render_map(d: &mut RaylibDrawHandle, map: &Map) -> Result<(), std::num::ParseIntError> {

    let color_cell    = Color::from_hex("2d8fb5")?;
    let color_cell_bg = Color::from_hex("2e2e2e")?;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {

            let color = if *cell == 1 { color_cell } else { color_cell_bg };

            d.draw_rectangle(
                x as i32 * CELL_SIZE + OFFSET.x as i32,
                y as i32 * CELL_SIZE + OFFSET.y as i32,
                CELL_SIZE,
                CELL_SIZE,
                color
            );

        }
    }

    Ok(())

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let player = Player::new();

    let map: Map = [
        [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
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

    }

    Ok(())

}
