use std::rc::Rc;

use raylib::prelude::*;

use crate::TextureDrawHandle;


pub type Texture = Rc<[[Color; TEX_WIDTH]; TEX_HEIGHT]>;
pub const TEX_WIDTH:  usize = 10;
pub const TEX_HEIGHT: usize = TEX_WIDTH;


type CellType = Option<Texture>;
pub const MAP_WIDTH:  usize = 10;
pub const MAP_HEIGHT: usize = 15;
pub const MAP_CELL_SIZE: i32 = 25;

pub struct Map([[CellType; MAP_WIDTH]; MAP_HEIGHT]);

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
            for (x, _cell) in row.iter().enumerate() {
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
