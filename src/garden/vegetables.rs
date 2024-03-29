use image::GenericImageView;
use image::Rgba;
use image::RgbaImage;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn vegetables(pid: u8) {
    let pwd = env::current_dir().unwrap();
    let asset_dir = PathBuf::from(pwd).join("assets");
    let img = image::open(PathBuf::from(&asset_dir).join("yqj.png")).expect("image not found");

    let mut color_map = HashMap::new();
    let mut max_color: Option<Rgba<u8>> = None;

    for pix in img.pixels() {
        let (_x, _y, color) = pix;
        color_map
            .entry(color)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        match max_color {
            None => max_color = Some(color),
            Some(c) => {
                if color_map.get(&c) < color_map.get(&color) {
                    max_color = Some(color);
                }
            }
        }
    }

    assert!(max_color.is_some(), "empty image");
    let m_color = max_color.unwrap();
    let count = color_map.get(&m_color).unwrap();
    println!("max_color: {:?} count: {:?}", m_color, count);

    let mut new_image = RgbaImage::new(img.width(), img.height());
    for pix in img.pixels() {
        let (_x, _y, color) = pix;
        let [_r, _g, _b, a] = color.0;
        new_image.put_pixel(
            _x,
            _y,
            if pid == 1 {
                Rgba([255 - _r, 255 - _g, 0, a])
            } else if pid == 2 {
                Rgba([0, _g, _b, a])
            } else if pid == 3 {
                Rgba([_r, 0, _b, a])
            } else if pid == 4 {
                Rgba([_r, _g, 0, a])
            } else {
                Rgba([
                    ((_r as f64) * 0.3 as f64).round() as u8,
                    ((_g as f64) * 0.59 as f64).round() as u8,
                    ((_b as f64) * 0.11 as f64).round() as u8,
                    a,
                ])
            },
        );
    }
    new_image
        .save(PathBuf::from(&asset_dir).join(format!("output-{pid}yqj.png")))
        .expect("save image faild");
}
