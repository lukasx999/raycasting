use raylib::prelude::*;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

// https://lodev.org/cgtutor/raycasting.html



const CELL_SIZE: i32 = 50;

//const OFFSET: Vector2 = Vector2::new(
//    (SCREEN_WIDTH  / 2 - CELL_SIZE * MAP_WIDTH  as i32 / 2) as f32,
//    (SCREEN_HEIGHT / 2 - CELL_SIZE * MAP_HEIGHT as i32 / 2) as f32
//);

const OFFSET: Vector2 = Vector2::new(10.0, 40.0);





type CellType = i32;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = MAP_WIDTH;

pub struct Map([[CellType; MAP_WIDTH]; MAP_HEIGHT]);

// TODO: load map from file

impl Map {
    pub fn new() -> Self {
        Self([
            [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 3, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 2, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 3, 0, 2, 0, 4, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
        ])
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellType {
        self.0[y][x]
    }

    pub fn render(&self, draw: &mut RaylibDrawHandle) {
        let color_cell_bg = Color::from_hex("2e2e2e").unwrap();

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = get_cell_color(*cell).unwrap_or(color_cell_bg);

                let rec_cell = Rectangle::new(
                    x as f32 * CELL_SIZE as f32 + OFFSET.x,
                    y as f32 * CELL_SIZE as f32 + OFFSET.y,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                );

                draw.draw_rectangle_rec(rec_cell, color);
                draw.draw_rectangle_lines_ex(rec_cell, 1.0, color_cell_bg.brightness(0.03));

            }
        }

        let map_border = Rectangle::new(
            OFFSET.x,
            OFFSET.y,
            MAP_WIDTH as f32 * CELL_SIZE as f32,
            MAP_HEIGHT as f32 * CELL_SIZE as f32
        );

        draw.draw_rectangle_lines_ex(map_border, 1.0, Color::WHITESMOKE);

    }
}


// Helper functions

fn map_connect_points(d: &mut RaylibDrawHandle, p1: Vector2, p2: Vector2, color: Color) {
    let size = 5.0;
    d.draw_line_ex(
        p1 * CELL_SIZE as f32 + OFFSET,
        p2 * CELL_SIZE as f32 + OFFSET,
        size,
        color
    );
}

fn map_point(d: &mut RaylibDrawHandle, center: Vector2, size: f32, color: Color) {
    d.draw_circle_v(
        center * CELL_SIZE as f32 + OFFSET,
        size,
        color
    );
}

fn map_square(d: &mut RaylibDrawHandle, pos: Vector2, color: Color) {
    d.draw_rectangle_v(
        pos * CELL_SIZE as f32 + OFFSET,
        Vector2::new(CELL_SIZE as f32, CELL_SIZE as f32),
        color
    );
}

fn get_cell_color(cell: i32) -> Option<Color> {
    match cell {
        1 => Some(Color::from_hex("585a5c").unwrap()),
        2 => Some(Color::from_hex("164c82").unwrap()),
        3 => Some(Color::from_hex("fcba03").unwrap()),
        4 => Some(Color::from_hex("b82d23").unwrap()),
        0 => None,
        _ => panic!("Unknown cell type"),
    }
}





const PLAYER_STEP: f32 = 0.3;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
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
        use std::f32::consts::PI;

        use Direction as D;
        match dir {
            D::North => self.position += self.direction * PLAYER_STEP,
            D::South => self.position -= self.direction * PLAYER_STEP,
            D::East  => self.position += self.direction.rotated(PI / 2.0) * PLAYER_STEP,
            D::West  => self.position -= self.direction.rotated(PI / 2.0) * PLAYER_STEP,
        }
    }

    pub fn rotate(&mut self, counter_clockwise: bool, using_mouse: bool) {
        let mut step = if counter_clockwise { -PLAYER_STEP } else { PLAYER_STEP };
        if using_mouse {
            step /= 3.0;
        }
        self.direction.rotate(step);
        self.plane.rotate(step);
    }

    pub fn change_fov_len(&mut self, dec_else_inc: bool) {
        let step = if dec_else_inc { -PLAYER_STEP } else { PLAYER_STEP };
        self.direction.x += step;
    }

    pub fn change_fov_width(&mut self, dec_else_inc: bool) {
        let step = if dec_else_inc { -PLAYER_STEP } else { PLAYER_STEP };
        self.plane.y += step;
    }

    pub fn render(&self, draw: &mut RaylibDrawHandle) {
        let color = Color::from_hex("3888c2").unwrap();

        let pos    = self.position;
        let dir    = self.position + self.direction;
        let plane1 = dir + self.plane;
        let plane2 = dir - self.plane;

        map_connect_points(draw, dir, plane1, color); // left
        map_connect_points(draw, dir, plane2, color); // right
        map_connect_points(draw, pos, plane2, color); // left-diagonal
        map_connect_points(draw, pos, plane1, color); // right-diagonal

        map_point(draw, pos, 10.0, color);

    }

}

// TODO: refactor

pub fn cast_rays(draw: &mut RaylibDrawHandle, player: &Player, map: &Map) {

    //for x in 0..=SCREEN_WIDTH {
    for x in (0..=SCREEN_WIDTH).step_by(CELL_SIZE as usize) {
    //let x = 0; { // only 1 ray for testing

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;

        let ray_dir = player.direction + player.plane * camera_x;
        let pos = player.position;

        // the length of a step needed to get to the x/y edge of the next cell
        // the formula is a simplified version of the pythagorean theorem
        // => slope = ray_dir.y / ray_dir.x
        // => sqrt(1 + slope.pow(2))
        let delta_dist = Vector2::new(
            ray_dir.x.recip().abs(),
            ray_dir.y.recip().abs(),
        );

        // the current cell of the map
        // floating point value gets removed from player position
        // has to be isize, because we later cast step to usize,
        // and things will be messed up if step is negative
        let (mut map_x, mut map_y) = (pos.x as isize, pos.y as isize);

        // IMPORTANT: the x and y components of side_dist and delta_dist
        // are both euclidean distances, not x/y coordinates

        // step for incrementing map_x/y
        let mut step = Vector2::zero();

        // initial distance from player position to end of first cell
        // will get incremented by delat_dist
        let mut side_dist = Vector2::zero();


        // TODO: why does multiplying perpendicular distance by delta_dist
        // yield euclidean distance?
        if ray_dir.x < 0.0 {
            step.x = -1.0;
            side_dist.x = (pos.x - map_x as f32) * delta_dist.x;
        } else {
            step.x = 1.0;
            side_dist.x = (map_x as f32 + 1.0 - pos.x) * delta_dist.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1.0;
            side_dist.y = (pos.y - map_y as f32) * delta_dist.y;
        } else {
            step.y = 1.0;
            side_dist.y = (map_y as f32 + 1.0 - pos.y) * delta_dist.y;
        }

        let mut side: i32;

        // DDA
        loop {

            if side_dist.x < side_dist.y {
                //map_connect_points(draw, pos, pos + ray_dir * side_dist.x, color_ray);
                side_dist.x += delta_dist.x;
                map_x += step.x as isize;
                side = 0;

            } else {
                //map_connect_points(draw, pos, pos + ray_dir * side_dist.y, color_ray);
                side_dist.y += delta_dist.y;
                map_y += step.y as isize;
                side = 1;
            }

            // out of bounds check (no wall in sight)
            if map_x as usize >= MAP_WIDTH || map_y as usize >= MAP_HEIGHT {
                break;
            }

            let cell = map.get_cell(map_x as usize, map_y as usize);
            let color = get_cell_color(cell);

            if let Some(mut color) = color {
                //map_square(draw, Vector2::new(map_x as f32, map_y as f32), Color::PURPLE);

                // make x-side slighty darker
                if side == 0 {
                    color = color.brightness(0.1);
                }

                // substract delta_dist once, because the dda algorithm went one cell too far
                let perp_wall_dist = if side == 0 {
                    side_dist.x - delta_dist.x
                } else {
                    side_dist.y - delta_dist.y
                };

                let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

                let start = SCREEN_HEIGHT / 2 - line_height / 2;
                let start = start.clamp(0, std::i32::MAX);

                draw.draw_rectangle(x, start, CELL_SIZE, line_height, color);

                break;
            }

        }


    }

}
