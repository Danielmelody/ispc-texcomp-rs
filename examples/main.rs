use ddsfile::NewDxgiParams;
use image::{GenericImageView, ImageBuffer};
use std::fs::File;
use std::path::Path;

use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, Dds, DxgiFormat};

fn main() {
    let imgpath = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("examples/rust.png".to_string());

    let img_origin = image::open(&Path::new(&imgpath)).unwrap();

    let (origin_width, origin_height) = img_origin.dimensions();
    let (round_width, round_height) = ((origin_width + 3) / 4 * 4, (origin_height + 3) / 4 * 4);
    println!("Width is {}", origin_width);
    println!("Height is {}", origin_height);
    println!("ColorType is {:?}", img_origin.color());

    let alpha_mode = match img_origin.color() {
        image::ColorType::Rgb8 => AlphaMode::Opaque,
        image::ColorType::Rgba8 => AlphaMode::Straight,
        _ => AlphaMode::Unknown,
    };

    img_origin.flipv();

    // We need rgba buffer to compress texture
    let mut img_rounded = ImageBuffer::new(round_width, round_height);
    println!("Converting RGB -> RGBA");
    for x in (0_u32..origin_width).into_iter() {
        for y in (0_u32..origin_height).into_iter() {
            let pixel = img_origin.get_pixel(x, y);
            img_rounded.put_pixel(x, y, pixel);
        }
    }

    let block_count = ispc_texcomp::cal_block_count(origin_width, origin_height, 4, 4);
    println!("Block count: {}", block_count);

    let dxgi_param = NewDxgiParams {
        height: origin_height,
        width: origin_width,
        depth: Some(1),
        format: DxgiFormat::BC7_UNorm,
        mipmap_levels: Some(1),
        array_layers: Some(1),
        caps2: Some(Caps2::empty()),
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode,
    };

    let mut dds = Dds::new_dxgi(dxgi_param).unwrap();

    let surface = ispc_texcomp::RgbaSurface {
        width: round_width,
        height: round_height,
        stride: round_width * 4,
        data: &img_rounded,
    };

    println!("Compressing to BC7...");
    ispc_texcomp::bc7::compress_blocks_into(
        &ispc_texcomp::bc7::alpha_fast_settings(),
        &surface,
        &mut dds.get_mut_data(0 /* layer */).unwrap(),
    );
    println!("  Done!");

    let outpath = imgpath.replace(".png", ".dds").replace(".jpg", ".dds");

    println!("Saving {} file", outpath);
    let mut dds_file = File::create(outpath).unwrap();
    dds.write(&mut dds_file).expect("Failed to write dds file");
}
