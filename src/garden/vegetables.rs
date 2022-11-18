use image::GenericImageView;
use image::Rgba;
use image::RgbaImage;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn vegetables() {
    let pwd = env::current_dir().unwrap();
    let asset_dir = PathBuf::from(pwd).join("assets");
    println!("{}", asset_dir.display());
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

    // match max_color {
    //     Some(color) => {
    //         println!(
    //             "max_color: {:?} count: {:?}",
    //             color,
    //             match color_map.get(&color) {
    //                 Some(count) => {
    //                     count
    //                 }
    //                 _ => {
    //                     &1
    //                 }
    //             }
    //         )
    //     }
    //     None => println!("No max color found"),
    // }

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
            // Rgba([255 - _r, 255 - _g, 0, a]),
            Rgba([_r, 0, _b, a]),
            // Rgba([_r, _g, 0, a]),
        );
    }
    new_image
        .save(PathBuf::from(&asset_dir).join("output-yqj.png"))
        .expect("save image faild");
}
