extern crate image;

use image::*;
use std::path::Path;

fn main() {
    // 灰色のドット１つ分だぜ☆（＾～＾）
    {
        let buffer: &[u8] = &[128, 128, 128, 128];
        image::save_buffer(
            &Path::new("output/gray-1dot.png"),
            buffer,
            1,
            1,
            ColorType::Rgba8,
        )
        .unwrap();
    }

    // 次はベクターで☆（＾～＾）
    {
        let gray = &Dot {
            r: 128,
            g: 128,
            b: 128,
            a: 128,
        }
        .array();
        let red = &Dot {
            r: 204,
            g: 51,
            b: 51,
            a: 255,
        }
        .array();
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(red);
        vec.extend_from_slice(gray);
        vec.extend_from_slice(gray);
        vec.extend_from_slice(red);
        image::save_buffer(
            &Path::new("output/ichimatsu.png"),
            &vec,
            2,
            2,
            ColorType::Rgba8,
        )
        .unwrap();
    }

    // サイズ指定で☆（＾～＾）
    {
        let frame = Frame::new(32, 64);
        image::save_buffer(
            &Path::new("output/32x64.png"),
            &frame.to_vec(),
            frame.width,
            frame.height,
            ColorType::Rgba8,
        )
        .unwrap();
    }
}

struct Frame {
    pub dots: Vec<Dot>,
    pub width: u32,
    pub height: u32,
}
impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Frame {
            dots: vec![Dot::default(); (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for dot in &self.dots {
            vec.extend_from_slice(&dot.array());
        }
        vec
    }
}

#[derive(Clone)]
struct Dot {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Dot {
    pub fn array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
impl Default for Dot {
    fn default() -> Self {
        Dot {
            r: 0,
            g: 128,
            b: 128,
            a: 255,
        }
    }
}
