use raylib::prelude::*;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, Framebuffer, Stripe};
use crate::player::Player;
use crate::map::{
    Map,
    MAP_WIDTH,
    MAP_HEIGHT,
    TEX_WIDTH,
    TEX_HEIGHT,
    Texture
};

// https://lodev.org/cgtutor/raycasting.html

pub const OFFSET: Vector2 = Vector2::new(10.0, 40.0);

// determines which side of a cell was hit by the ray
#[derive(Debug, Clone, Copy, PartialEq)]
enum Side { X, Y }


pub fn cast_stripe() {
    // TODO: this
}

#[derive(Debug, Clone)]
pub struct Raycaster {
    x:          i32,
    pos:        Vector2,
    delta_dist: Vector2,
    // initial distance from player position to end of first cell
    // will get incremented by delta_dist
    side_dist:  Vector2,
    ray_dir:    Vector2,
    // step for incrementing map_x/y
    step:       Vector2,
    mapx:       isize,
    mapy:       isize,
}

impl Raycaster {

    pub fn new() -> Self {
        Self {
            x:          0,
            pos:        Vector2::zero(),
            delta_dist: Vector2::zero(),
            side_dist:  Vector2::zero(),
            ray_dir:    Vector2::zero(),
            step:       Vector2::zero(),
            mapx:       0,
            mapy:       0,
        }
    }

    fn init(&mut self, x: i32, player: &Player) {

        self.x = x;
        self.pos = player.position;

        /* -1.0 <-> 0.0 <-> 1.0 */
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;

        self.ray_dir = player.direction + player.plane * camera_x;

        // the length of a step needed to get to the x/y edge of the next cell
        self.delta_dist = Vector2::new(
            self.ray_dir.x.recip().abs(),
            self.ray_dir.y.recip().abs(),
        );

        // the current cell of the map
        // floating point value gets removed from player position
        // has to be isize, because we later cast step to usize,
        // and things will be messed up if step is negative
        (self.mapx, self.mapy) = (self.pos.x as isize, self.pos.y as isize);

        if self.ray_dir.x < 0.0 {
            self.step.x = -1.0;
            self.side_dist.x = (self.pos.x - self.mapx as f32) * self.delta_dist.x;
        } else {
            self.step.x = 1.0;
            self.side_dist.x = (self.mapx as f32 + 1.0 - self.pos.x) * self.delta_dist.x;
        }

        if self.ray_dir.y < 0.0 {
            self.step.y = -1.0;
            self.side_dist.y = (self.pos.y - self.mapy as f32) * self.delta_dist.y;
        } else {
            self.step.y = 1.0;
            self.side_dist.y = (self.mapy as f32 + 1.0 - self.pos.y) * self.delta_dist.y;
        }

    }

    fn dda(&mut self, map: &Map, stripe: Stripe) {

        let Self { delta_dist, side_dist, mapx, mapy, step, .. } = self;

        loop {

            // whether the X or Y side of a cell was hit
            let side: Side;

            //let mut texture_draw = draw.begin_texture_mode(&thread, texture_minimap);
            //let color_ray = Color::from_hex("4a4949").unwrap();

            if side_dist.x < side_dist.y {
                //map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.x, color_ray);
                side_dist.x += delta_dist.x;
                *mapx += step.x as isize;
                side = Side::X;

            } else {
                //map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.y, color_ray);
                side_dist.y += delta_dist.y;
                *mapy += step.y as isize;
                side = Side::Y;
            }

            // out of bounds check (no wall in sight)
            if *mapx as usize >= MAP_WIDTH
            || *mapy as usize >= MAP_HEIGHT {
                break;
            }

            // Ray collision with wall
            let cell = map.get_cell(*mapx as usize, *mapy as usize);
            if let Some(texture) = cell {

                //map_square(&mut texture_draw, Vector2::new(mapx as f32, mapy as f32), Color::RED.brightness(0.3));
                //drop(texture_draw);

                self.render_texture(stripe, side, texture);
                break;
            }

        }
    }

    fn render_texture(&mut self, stripe: Stripe, side: Side, texture: &Texture) {

        // substract delta_dist once, because the dda algorithm went one cell too far
        let perp_wall_dist = match side {
            Side::X => self.side_dist.x - self.delta_dist.x,
            Side::Y => self.side_dist.y - self.delta_dist.y,
        };

        let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

        let start = (SCREEN_HEIGHT / 2 - line_height / 2)
            .clamp(0, std::i32::MAX);

        // the exact position of where the wall was hit (for textures)
        let mut wallx = match side {
            Side::X => self.pos.y + perp_wall_dist * self.ray_dir.y,
            Side::Y => self.pos.x + perp_wall_dist * self.ray_dir.x,
        };
        wallx -= wallx.floor(); // 0.0 <-> 1.0

        let tex_x = wallx * TEX_WIDTH as f32;
        let step = TEX_HEIGHT as f32 / line_height as f32;
        let mut tex_y = 0.0;

        let mut stripe = stripe.lock().unwrap();
        for y in start..start+line_height {

            let mut color = texture[tex_y as usize][tex_x as usize];

            if side == Side::Y {
                color = color.brightness(-0.3);
            }

            stripe[y as usize] = color;
            //draw.draw_rectangle(self.x, y, 1, 1, color);
            tex_y += step;
        }
    }

    pub fn render_stripe(x: i32, stripe: Stripe, player: &Player, map: &Map) {
        let mut s = Self::new();

        // Prepare values for DDA algorithm
        s.init(x, player);

        // Calculate ray length and render textures
        s.dda(map, stripe);
    }

    pub fn cast_rays(fb: &mut Framebuffer, player: &Player, map: &Map) {

        std::thread::scope(|s| {
            for x in 0..SCREEN_WIDTH {
                let stripe = fb.0[x as usize].clone();

                s.spawn(move || {
                    Self::render_stripe(x, stripe, player, map);
                });

            }

        });

    }

}
