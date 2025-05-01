use axum::{
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use image::{ImageFormat, Rgb, RgbImage};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
pub struct TerrainParams {
    pub rooms: f64,
    pub seed: u32,
}

pub async fn generate_terrain(Query(params): Query<TerrainParams>) -> impl IntoResponse {
    const W: u32 = 256;
    const H: u32 = 256;

    let continents = Fbm::<Perlin>::new(params.seed)
        .set_octaves(6)
        .set_persistence(0.5)
        .set_lacunarity(2.0)
        .set_frequency(1.0);

    let scale = 1.0 + params.rooms / 10.0;
    let sea_level = 0.0_f64;

    let mut img = RgbImage::new(W, H);
    for y in 0..H {
        for x in 0..W {
            let nx = (x as f64 / W as f64 - 0.5) * scale;
            let ny = (y as f64 / H as f64 - 0.5) * scale;
            let v = continents.get([nx, ny, 0.0]);

            let pixel = if v <= sea_level {
                let depth = ((sea_level - v) / (sea_level + 1.0)).clamp(0.0, 1.0);
                let r = (70.0 + (0.0 - 70.0) * depth) as u8;
                let g = (130.0 + (0.0 - 130.0) * depth) as u8;
                let b = (180.0 + (255.0 - 180.0) * depth) as u8;
                Rgb([r, g, b])
            } else {
                let h = ((v - sea_level) / (1.0 - sea_level)).clamp(0.0, 1.0);
                if h < 0.25 {
                    Rgb([238, 214, 175]) 
                } else if h < 0.50 {
                    Rgb([34, 139, 34]) 
                } else if h < 0.75 {
                    Rgb([139, 137, 137]) 
                } else {
                    Rgb([255, 250, 250]) 
                }
            };

            img.put_pixel(x, y, pixel);
        }
    }

    let mut buf = Cursor::new(Vec::new());
    if image::DynamicImage::ImageRgb8(img)
        .write_to(&mut buf, ImageFormat::Png)
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let png_data = buf.into_inner();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::from(png_data))
        .unwrap()
}
