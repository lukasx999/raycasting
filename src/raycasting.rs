use raylib::prelude::*;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
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

// TODO: make this work
const PIXEL_SIZE: usize = 10;

// TODO: fix warped textures when block gets greater than screen

// determines which side of a cell was hit by the ray
#[derive(Debug, Clone, Copy, PartialEq)]
enum Side { X, Y }


fn render_texture(
    draw:           &mut impl RaylibDraw,
    x:              i32,
    side:           Side,
    pos:            Vector2,
    ray_dir:        Vector2,
    perp_wall_dist: f32,
    texture:        Texture
) {

    let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

    let start = (SCREEN_HEIGHT / 2 - line_height / 2)
        .clamp(0, std::i32::MAX);

    // the exact position of where the wall was hit (for textures)
    let mut wallx = match side {
        Side::X => pos.y + perp_wall_dist * ray_dir.y,
        Side::Y => pos.x + perp_wall_dist * ray_dir.x,
    };
    wallx -= wallx.floor(); // 0.0 <-> 1.0

    let tex_x = wallx * TEX_WIDTH as f32;
    let step = TEX_HEIGHT as f32 / line_height as f32;
    let mut tex_y = 0.0;

    //for y in (start..start+line_height).step_by(PIXEL_SIZE) {
    for y in start..start+line_height {

        let mut color = texture[tex_y as usize][tex_x as usize];

        if side == Side::Y {
            color = color.brightness(-0.3);
        }

        //draw.draw_rectangle(x, y, 1, 1, color);
        draw.draw_rectangle(x, y, PIXEL_SIZE as i32, 1, color);
        tex_y += step;
    }
}


fn render_stripe(draw: &mut impl RaylibDraw, x: i32, player: &Player, map: &Map) {

    /* INIT */

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

    // initial distance from player position to end of first cell
    // will get incremented by delta_dist
    let mut side_dist = Vector2::zero();

    // step for incrementing map_x/y
    let mut step = Vector2::zero();

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

    /* DDA */
    loop {

        // whether the X or Y side of a cell was hit
        let side: Side;

        //let mut texture_draw = draw.begin_texture_mode(&thread, texture_minimap);
        //let color_ray = Color::from_hex("4a4949").unwrap();

        if side_dist.x < side_dist.y {
            //map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.x, color_ray);
            side_dist.x += delta_dist.x;
            mapx += step.x as isize;
            side = Side::X;

        } else {
            //map_connect_points(&mut texture_draw, pos, pos + ray_dir * side_dist.y, color_ray);
            side_dist.y += delta_dist.y;
            mapy += step.y as isize;
            side = Side::Y;
        }

        // out of bounds check (no wall in sight)
        if mapx as usize >= MAP_WIDTH || mapy as usize >= MAP_HEIGHT {
            break;
        }

        // Ray collision with wall
        let cell = map.get_cell(mapx as usize, mapy as usize);
        if let Some(texture) = cell {

            //map_square(&mut texture_draw, Vector2::new(mapx as f32, mapy as f32), Color::RED.brightness(0.3));
            //drop(texture_draw);

            // substract delta_dist once, because the dda algorithm went one cell too far
            let perp_wall_dist = match side {
                Side::X => side_dist.x - delta_dist.x,
                Side::Y => side_dist.y - delta_dist.y,
            };

            render_texture(draw, x, side, pos, ray_dir, perp_wall_dist, texture.clone());
            break;
        }

    }

}

pub fn cast_rays(draw: &mut impl RaylibDraw, player: &Player, map: &Map) {
    for x in (0..SCREEN_WIDTH).step_by(PIXEL_SIZE) {
        render_stripe(draw, x, player, map);
    }
}
