use raylib::prelude::*;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, TextureDrawHandle};

// https://lodev.org/cgtutor/raycasting.html

// Texture dimensions
const TEX_WIDTH: usize = 5;
const TEX_HEIGHT: usize = TEX_WIDTH;
type Texture = [[Color; TEX_WIDTH]; TEX_HEIGHT];

pub const CELL_SIZE: i32 = 25;

// if resolution is too high (low), frames will drop because of begin_texture_mode()
// being created and dropped multiple times every frame
const RESOLUTION: i32 = 10;

//const OFFSET: Vector2 = Vector2::new(
//    (SCREEN_WIDTH  / 2 - CELL_SIZE * MAP_WIDTH  as i32 / 2) as f32,
//    (SCREEN_HEIGHT / 2 - CELL_SIZE * MAP_HEIGHT as i32 / 2) as f32
//);

pub const OFFSET: Vector2 = Vector2::new(10.0, 40.0);



type CellType = i32;
pub const MAP_WIDTH:  usize = 10;
pub const MAP_HEIGHT: usize = 15;

pub struct Map([[CellType; MAP_WIDTH]; MAP_HEIGHT]);

// TODO: load map from file

impl Map {
    pub fn new() -> Self {
        Self([
            [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 4, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 2, 3, 4, 4, 3, 2, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
            [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
        ])
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellType {
        self.0[y][x]
    }

    pub fn render(&self, draw: &mut TextureDrawHandle) {
         let color_cell_bg = Color::from_hex("2e2e2e").unwrap();

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = get_cell_color(*cell).unwrap_or(color_cell_bg);

                let rec_cell = Rectangle::new(
                    x as f32 * CELL_SIZE as f32,
                    y as f32 * CELL_SIZE as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                );

                draw.draw_rectangle_rec(rec_cell, color);
                draw.draw_rectangle_lines_ex(rec_cell, 1.0, color_cell_bg.brightness(0.03));

            }
        }

        let map_border = Rectangle::new(
            0.0,
            0.0,
            MAP_WIDTH as f32 * CELL_SIZE as f32,
            MAP_HEIGHT as f32 * CELL_SIZE as f32
        );

        draw.draw_rectangle_lines_ex(map_border, 1.0, Color::WHITESMOKE);

    }
}


// Helper functions

fn map_connect_points(d: &mut impl RaylibDraw, p1: Vector2, p2: Vector2, color: Color) {
    let size = 3.0;
    d.draw_line_ex(
        p1 * CELL_SIZE as f32,
        p2 * CELL_SIZE as f32,
        size,
        color
    );
}

fn map_point(d: &mut TextureDrawHandle, center: Vector2, size: f32, color: Color) {
    d.draw_circle_v(
        center * CELL_SIZE as f32,
        size,
        color
    );
}

fn map_square(d: &mut TextureDrawHandle, pos: Vector2, color: Color) {
    d.draw_rectangle_v(
        pos * CELL_SIZE as f32,
        Vector2::new(CELL_SIZE as f32, CELL_SIZE as f32),
        color
    );
}



const PLAYER_STEP: f32 = 0.5;

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
            position:  Vector2::new(5.0, 7.0),
            direction: Vector2::new(1.0, 0.0),
            plane:     Vector2::new(0.0, 0.66),
        }
    }

    pub fn move_(&mut self, dir: Direction) {
        use std::f32::consts::FRAC_PI_2;

        let step = PLAYER_STEP / 10.0;

        use Direction as D;
        match dir {
            D::North => self.position += self.direction * step,
            D::South => self.position -= self.direction * step,
            D::East  => self.position += self.direction.rotated(FRAC_PI_2) * step,
            D::West  => self.position -= self.direction.rotated(FRAC_PI_2) * step,
        }
    }

    pub fn rotate(&mut self, counter_clockwise: bool) {
        let factor = 15.0;
        let step = if counter_clockwise { -PLAYER_STEP } else { PLAYER_STEP } / factor;
        self.direction.rotate(step);
        self.plane.rotate(step);
    }

    pub fn increase_fov(&mut self, dec: bool) {
        let factor = 2.0;
        let step = PLAYER_STEP / factor;

        if dec {
            self.direction -= self.direction * step;
        } else {
            self.direction += self.direction * step;
        }
    }

    pub fn increase_fov_width(&mut self, dec: bool) {
        let factor = 2.0;
        let step = PLAYER_STEP / factor;

        if dec {
            self.plane -= self.plane * step;
        } else {
            self.plane += self.plane * step;
        }
    }

    pub fn render(&self, draw: &mut TextureDrawHandle) {
        let color = Color::WHITESMOKE;

        let pos    = self.position;
        let dir    = self.position + self.direction;
        let plane1 = dir + self.plane;
        let plane2 = dir - self.plane;

        map_connect_points(draw, dir, plane1, color); // left
        map_connect_points(draw, dir, plane2, color); // right
        map_connect_points(draw, pos, plane2, color); // left-diagonal
        map_connect_points(draw, pos, plane1, color); // right-diagonal

        map_point(draw, pos, 5.0, color);

    }

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

// determines which side of a cell was hit by the ray
#[derive(Debug, Clone, Copy)]
enum Side { X, Y }

// TODO: render output into buffer, so texture draw handle doesnt have to be
// created and destroyed every frame

pub fn render_world_3d(
    draw:   &mut RaylibDrawHandle,
    thread: &RaylibThread,
    player: &Player,
    map:    &Map,
    texture_minimap: &mut RenderTexture2D,
) {

    {
        use Color as C;
        let brick: Texture = [
            [ C::BLACK, C::BLACK, C::BLACK, C::BLACK, C::BLACK ],
            [ C::WHITE, C::RED,   C::RED,   C::BLACK, C::BLUE  ],
            [ C::WHITE, C::RED,   C::RED,   C::BLACK, C::BLUE  ],
            [ C::BLACK, C::BLACK, C::BLACK, C::BLACK, C::BLACK ],
            [ C::BLACK, C::BLACK, C::BLACK, C::BLACK, C::BLACK ],
        ];
    }



    for x in (0..=SCREEN_WIDTH).step_by(RESOLUTION as usize) {
        let pos = player.position;

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;

        let ray_dir = player.direction + player.plane * camera_x;

        // the length of a step needed to get to the x/y edge of the next cell
        let delta_dist = Vector2::new(
            ray_dir.x.recip().abs(),
            ray_dir.y.recip().abs(),
        );

        // the current cell of the map
        // floating point value gets removed from player position
        // has to be isize, because we later cast step to usize,
        // and things will be messed up if step is negative
        let (mut mapx, mut mapy) = (pos.x as isize, pos.y as isize);

        // step for incrementing map_x/y
        let mut step = Vector2::zero();

        // initial distance from player position to end of first cell
        // will get incremented by delta_dist
        let mut side_dist = Vector2::zero();


        if ray_dir.x < 0.0 {
            step.x = -1.0;
            side_dist.x = (pos.x - mapx as f32) * delta_dist.x;
        } else {
            step.x = 1.0;
            side_dist.x = (mapx as f32 + 1.0 - pos.x) * delta_dist.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1.0;
            side_dist.y = (pos.y - mapy as f32) * delta_dist.y;
        } else {
            step.y = 1.0;
            side_dist.y = (mapy as f32 + 1.0 - pos.y) * delta_dist.y;
        }



        // DDA
        loop {

            let side: Side;

            let mut texture_draw = draw.begin_texture_mode(&thread, texture_minimap);
            let color_ray = Color::from_hex("4a4949").unwrap();

            if side_dist.x < side_dist.y {
                map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.x, color_ray);
                side_dist.x += delta_dist.x;
                mapx += step.x as isize;
                side = Side::X;

            } else {
                map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.y, color_ray);
                side_dist.y += delta_dist.y;
                mapy += step.y as isize;
                side = Side::Y;
            }

            // out of bounds check (no wall in sight)
            if mapx as usize >= MAP_WIDTH || mapy as usize >= MAP_HEIGHT {
                break;
            }

            let cell = map.get_cell(mapx as usize, mapy as usize);
            let color = get_cell_color(cell);

            if let Some(mut color) = color {

                map_square(&mut texture_draw, Vector2::new(mapx as f32, mapy as f32), color.brightness(0.3));
                drop(texture_draw);

                // make x-side slighty darker
                if let Side::X = side {
                    color = color.brightness(0.1);
                }

                // substract delta_dist once, because the dda algorithm went one cell too far
                let perp_wall_dist = match side {
                    Side::X => side_dist.x - delta_dist.x,
                    Side::Y => side_dist.y - delta_dist.y,
                };

                let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

                let start = SCREEN_HEIGHT / 2 - line_height / 2;
                let start = start.clamp(0, std::i32::MAX);

                draw.draw_rectangle(x, start, RESOLUTION, line_height, color);

                break;
            }

        }


    }

}
