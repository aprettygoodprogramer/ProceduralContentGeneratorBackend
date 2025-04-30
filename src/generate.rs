use axum::{
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use image::{DynamicImage, GrayImage, ImageFormat, Luma};
use noise::{NoiseFn, Perlin};
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

    let perlin = Perlin::new(params.seed);

    let scale = 1.0 + params.rooms / 10.0;

    let mut img: GrayImage = GrayImage::new(W, H);
    for y in 0..H {
        for x in 0..W {
            let nx = x as f64 / W as f64;
            let ny = y as f64 / H as f64;
            let v = perlin.get([nx * scale, ny * scale, 0.0]);
            let px = (((v + 1.0) * 0.5) * 255.0).clamp(0.0, 255.0) as u8;
            img.put_pixel(x, y, Luma([px]));
        }
    }

    let dyn_img = DynamicImage::ImageLuma8(img);
    let mut buf = Cursor::new(Vec::new());
    if dyn_img.write_to(&mut buf, ImageFormat::Png).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let png_data = buf.into_inner();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::from(png_data))
        .unwrap()
}
