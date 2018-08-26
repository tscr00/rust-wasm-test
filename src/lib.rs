#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point2d {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }

    pub fn white() -> Color {
        Color { r: 255, g: 255, b: 255, a: 255 }
    }
}

/// Represents a two-dimensional canvas as flat array of 4 8-bit unsigned integers (RGBA values).
#[wasm_bindgen]
pub struct FlatCanvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl FlatCanvas {

    pub fn new(width: i32, height: i32) -> FlatCanvas {
        assert!(width > 0 && height > 0);

        let pixels = vec![Color::white(); (width*height) as usize];
        FlatCanvas { width, height, pixels }
    }

    pub fn canvas(&self) -> *const Color {
        self.pixels.as_ptr()
    }

    fn contains_point(&self, point: &Point2d) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    fn reset(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = Color::white();
        }
    }

    pub fn draw_point(&mut self, point: Point2d, color: Color) {
        if !self.contains_point(&point) {
            return
        }

        let position = point.y * self.width + point.x;

        self.pixels[position as usize] = color;
    }

    pub fn draw_line(&mut self, from: Point2d, to: Point2d, color: Color) {
        let Point2d { x: x1, y: y1 } = from;
        let Point2d { x: x2, y: y2 } = to;

        let dx = x2 - x1;
        let dy = y2 - y2;

        let mut steep = false;

        if dy.abs() > dx.abs() {
            steep = true;
            let (x1, y1) = (y1, x1);
            let (x2, y2) = (y2, x2);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;

        let mut error = 0.5;
        let delta_error = ((dy as f32) / (dx as f32)).abs();

        let y_dir = if y1 < y2 {
            1
        } else {
            -1
        };

        let mut y = y1;

        for x in x1..=x2 {
            if steep {
                self.draw_point(Point2d { x: y, y: x }, color)
            } else {
                self.draw_point(Point2d { x: x, y: y}, color)
            }

            error += delta_error;

            if error >= 1.0 {
                error -= 1.0;
                y += y_dir;
            }
        }
    }
}
