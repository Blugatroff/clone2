use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
struct Rect {
    top_left_y: u32,
    top_left_x: u32,
    down_right_x: u32,
    down_right_y: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct UvRect {
    top_left_x: f32,
    top_left_y: f32,
    down_right_x: f32,
    down_right_y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Texture {
    name: String,
    rect: UvRect,
}

fn main() {
    let mut args = std::env::args();

    let _ = args.next();
    let dst_path = args.next().unwrap();

    let mut textures: Vec<(String, DynamicImage)> = args
        .into_iter()
        .map(|path| {
            let mut path_buf = PathBuf::from(&path);
            path_buf.set_extension("");
            let name: String = String::from(path_buf.file_name().unwrap().to_str().unwrap());
            (name, image::open(path).unwrap())
        })
        .collect();

    let mut height = 0;
    let mut width = 0;

    for (_, texture) in &textures {
        width = width.max(texture.width());
        height += texture.height();
    }

    let mut atlas = image::DynamicImage::new_rgba8(width, height);

    let mut atlas_map = Vec::new();

    let mut y = 0;
    for (name, texture) in &mut textures {
        let src_rect = Rect {
            top_left_y: 0,
            top_left_x: 0,
            down_right_x: texture.width(),
            down_right_y: texture.height(),
        };
        let dst_rect = Rect {
            top_left_y: y,
            top_left_x: 0,
            down_right_x: texture.width(),
            down_right_y: y + texture.height(),
        };
        copy_image_to_image(texture, &mut atlas, src_rect, dst_rect);
        atlas_map.push(Texture {
            name: name.clone(),
            rect: UvRect {
                top_left_x: dst_rect.top_left_x as f32 / width as f32,
                top_left_y: dst_rect.top_left_y as f32 / height as f32,
                down_right_x: dst_rect.down_right_x as f32 / width as f32,
                down_right_y: dst_rect.down_right_y as f32 / height as f32,
            },
        });
        y += texture.height();
    }

    let json = serde_json::to_string_pretty(&atlas_map).unwrap();
    let mut dst_path = PathBuf::from(&dst_path);
    atlas.save_with_format(&dst_path, ImageFormat::Png).unwrap();
    dst_path.set_file_name("map.json");
    std::fs::write(dst_path, json).unwrap();
}

fn copy_image_to_image(
    src: &mut DynamicImage,
    dst: &mut DynamicImage,
    src_rect: Rect,
    dst_rect: Rect,
) {
    for x in src_rect.top_left_x..src_rect.down_right_x {
        for y in src_rect.top_left_y..src_rect.down_right_y {
            let pixel = src.get_pixel(x, y);
            let dx = x - src_rect.top_left_x;
            let dy = y - src_rect.top_left_y;
            let dst_x = dst_rect.top_left_x + dx;
            let dst_y = dst_rect.top_left_y + dy;
            if dst_x < dst_rect.down_right_x && dst_y < dst_rect.down_right_y {
                dst.put_pixel(dst_x, dst_y, pixel);
            }
        }
    }
}
