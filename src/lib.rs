#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

struct Point2d {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

trait Canvas {
    fn reset(&mut self);
    fn draw_point(&mut self, point: Point2d, color: Color);
    fn draw_line(&mut self, from: Point2d, to: Point2d, color: Color);
}

/// Represents a two-dimensional canvas as flat array of 4 8-bit unsigned integers (RGBA values).
#[wasm_bindgen]
pub struct FlatCanvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }
    pub fn white() -> Color {
        Color { r: 255, g: 255, b: 255, a: 255 }
    }
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
}

impl Canvas for FlatCanvas {

    fn reset(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = Color::white();
        }
    }

    fn draw_point(&mut self, point: Point2d, color: Color) {
        if !self.contains_point(&point) {
            return
        }

        let position = point.y * self.width + point.x;

        self.pixels[position as usize] = color;
    }

    fn draw_line(&mut self, from: Point2d, to: Point2d, color: Color) {
        unimplemented!();
    }
}
