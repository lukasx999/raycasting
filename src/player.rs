use raylib::prelude::*;

use crate::{MAP_CELL_SIZE, TextureDrawHandle};

pub const PLAYER_STEP: f32 = 0.5;

// Helper functions

fn map_connect_points(d: &mut impl RaylibDraw, p1: Vector2, p2: Vector2, color: Color) {
    let size = 3.0;
    d.draw_line_ex(
        p1 * MAP_CELL_SIZE as f32,
        p2 * MAP_CELL_SIZE as f32,
        size,
        color
    );
}

fn map_point(d: &mut TextureDrawHandle, center: Vector2, size: f32, color: Color) {
    d.draw_circle_v(
        center * MAP_CELL_SIZE as f32,
        size,
        color
    );
}

fn map_square(d: &mut TextureDrawHandle, pos: Vector2, color: Color) {
    d.draw_rectangle_v(
        pos * MAP_CELL_SIZE as f32,
        Vector2::new(MAP_CELL_SIZE as f32, MAP_CELL_SIZE as f32),
        color
    );
}


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
