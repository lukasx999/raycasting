use std::rc::Rc;

use raylib::prelude::*;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, TextureDrawHandle};

// https://lodev.org/cgtutor/raycasting.html



// Texture dimensions
const TEX_WIDTH: usize = 50;
const TEX_HEIGHT: usize = TEX_WIDTH;
type Texture = Rc<[[Color; TEX_WIDTH]; TEX_HEIGHT]>;

pub const MAP_CELL_SIZE: i32 = 25;

const RESOLUTION: i32 = 1;

//const OFFSET: Vector2 = Vector2::new(
//    (SCREEN_WIDTH  / 2 - CELL_SIZE * MAP_WIDTH  as i32 / 2) as f32,
//    (SCREEN_HEIGHT / 2 - CELL_SIZE * MAP_HEIGHT as i32 / 2) as f32
//);

pub const OFFSET: Vector2 = Vector2::new(10.0, 40.0);



type CellType = Option<Texture>;
pub const MAP_WIDTH:  usize = 10;
pub const MAP_HEIGHT: usize = 15;

pub struct Map([[CellType; MAP_WIDTH]; MAP_HEIGHT]);

// TODO: load map from file

impl Map {

    pub fn new() -> Self {
        let g = Some(Self::texture_gradient());
        let i = Some(Self::texture_stripes());
        let u = Some(Self::texture_stripes_h());
        let o = Some(Self::texture_outline());
        let a = Some(Self::texture_a());
        let n = None;

        Self([
            [ o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), i.clone(), n.clone(), g.clone(), n.clone(), o.clone(), n.clone(), a.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), u.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), a.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), g.clone(), g.clone(), g.clone(), g.clone(), n.clone(), o.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), n.clone(), o.clone() ],
            [ o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone(), o.clone() ],
        ])


    }

    fn texture_gradient() -> Texture {
        let mut tex = [[Color::BLACK; TEX_WIDTH]; TEX_HEIGHT];

        for (y, row) in tex.iter_mut().enumerate() {
            for cell in row {
                let value = y as f32 / TEX_WIDTH as f32;
                *cell = Color::BLUE.brightness(value);
            }
        }

        Rc::new(tex)
    }

    fn texture_stripes() -> Texture {
        let mut tex = [[Color::BLACK; TEX_WIDTH]; TEX_HEIGHT];
        let color_a = Color::from_hex("21c4ab").unwrap();
        let color_b = Color::from_hex("2168c4").unwrap();

        for row in tex.iter_mut() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = if x % 2 == 0 { color_a } else { color_b };
            }
        }

        Rc::new(tex)
    }

    fn texture_stripes_h() -> Texture {

        let mut tex = [[Color::BLACK; TEX_WIDTH]; TEX_HEIGHT];
        let color_a = Color::from_hex("e2b81f").unwrap();
        let color_b = Color::from_hex("b0bac4").unwrap();

        for (y, row) in tex.iter_mut().enumerate() {
            for cell in row.iter_mut() {
                *cell = if y % 2 == 0 { color_a } else { color_b };
            }
        }

        Rc::new(tex)
    }

    fn texture_a() -> Texture {

        let mut tex = [[Color::BLACK; TEX_WIDTH]; TEX_HEIGHT];
        let color_left = Color::RED;
        let color_right = Color::BLUE;

        for row in tex.iter_mut() {
            row[0] = color_left;
            row[row.len() - 1] = color_right;
        }

        Rc::new(tex)
    }

    fn texture_outline() -> Texture {

        let mut tex = [[Color::BLACK; TEX_WIDTH]; TEX_HEIGHT];
        let color_a = Color::from_hex("6c1efc").unwrap();
        let color_b = Color::RED;

        tex[tex.len()/2][tex[0].len() / 2] = color_b;

        for (y, row) in tex.iter_mut().enumerate() {
            row[0] = color_a;
            row[row.len() - 1] = color_a;

            if y == 0 {
                for cell in row.iter_mut() {
                    *cell = color_a;
                }
            }

            if y == row.len() - 1 {
                for cell in row.iter_mut() {
                    *cell = color_a;
                }
            }

        }

        Rc::new(tex)
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &CellType {
        &self.0[y][x]
    }

    pub fn render(&self, draw: &mut TextureDrawHandle) {
         let color_cell_bg = Color::from_hex("2e2e2e").unwrap();

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                //let color = get_cell_color(*cell).unwrap_or(color_cell_bg);
                let color = Color::BLUE;

                let rec_cell = Rectangle::new(
                    x as f32 * MAP_CELL_SIZE as f32,
                    y as f32 * MAP_CELL_SIZE as f32,
                    MAP_CELL_SIZE as f32,
                    MAP_CELL_SIZE as f32,
                );

                draw.draw_rectangle_rec(rec_cell, color);
                draw.draw_rectangle_lines_ex(rec_cell, 1.0, color_cell_bg.brightness(0.03));

            }
        }

        let map_border = Rectangle::new(
            0.0,
            0.0,
            MAP_WIDTH as f32 * MAP_CELL_SIZE as f32,
            MAP_HEIGHT as f32 * MAP_CELL_SIZE as f32
        );

        draw.draw_rectangle_lines_ex(map_border, 1.0, Color::WHITESMOKE);

    }
}


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

// determines which side of a cell was hit by the ray
#[derive(Debug, Clone, Copy, PartialEq)]
enum Side { X, Y }


fn raycasting_init(x: i32, player: &Player) -> (
    Vector2,
    Vector2,
    Vector2,
    Vector2,
    isize,
    isize
) {

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
    let (mapx, mapy) = (pos.x as isize, pos.y as isize);

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

    (ray_dir, side_dist, delta_dist, step, mapx, mapy)

}

fn render_texture(
    draw:           &mut RaylibDrawHandle,
    x:              i32,
    texture:        &Texture,
    pos:            Vector2,
    side:           Side,
    ray_dir:        Vector2,
    perp_wall_dist: f32,
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

    // TODO: find out what this stuff does
    //// correction for negative rays
    //// textures must be flipped
    //if side == Side::X && ray_dir.x > 0.0
    //|| side == Side::Y && ray_dir.y < 0.0 {
    //    tex_x = TEX_WIDTH as f32 - tex_x - 1.0;
    //}

    let step = TEX_HEIGHT as f32 / line_height as f32;
    let mut tex_y = 0.0;

    for y in (start..start+line_height).step_by(RESOLUTION as usize) {

        let mut color = texture[tex_y as usize][tex_x as usize];

        if side == Side::Y {
            color = color.brightness(-0.3);
        }

        //draw.draw_rectangle(x, y, 1, 1, color);
        draw.draw_rectangle(x, y, RESOLUTION, RESOLUTION, color);
        tex_y += step;
    }

}

fn dda(
    draw:          &mut RaylibDrawHandle,
    x:             i32,
    map:           &Map,
    pos:           Vector2,
    mut side_dist: Vector2,
    delta_dist:    Vector2,
    step:          Vector2,
    ray_dir:       Vector2,
    mut mapx:      isize,
    mut mapy:      isize
) {

    loop {

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

            render_texture(draw, x, &texture, pos, side, ray_dir, perp_wall_dist);
            break;
        }

    }

}

pub fn cast_rays(
    draw:   &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    player: &Player,
    map:    &Map,
    _texture_minimap: &mut RenderTexture2D,
) {

    for x in (0..=SCREEN_WIDTH).step_by(RESOLUTION as usize) {

        // Prepare values for DDA algorithm
        let (ray_dir, side_dist, delta_dist, step, mapx, mapy) = raycasting_init(x, player);

        // Calculate ray length and render textures
        dda(
            draw,
            x,
            map,
            player.position,
            side_dist,
            delta_dist,
            step,
            ray_dir,
            mapx,
            mapy
        );

    }

}
