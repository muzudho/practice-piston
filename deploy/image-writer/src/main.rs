extern crate image;

use image::*;
use std::path::Path;

fn main() {
    // 灰色のドット１つ分だぜ☆（＾～＾）
    let buffer: &[u8] = &[128, 128, 128, 128];

    // Save the buffer as "image.png"
    image::save_buffer(
        &Path::new("output/gray-1dot.png"),
        buffer,
        1,
        1,
        ColorType::Rgba8,
    )
    .unwrap();
}
