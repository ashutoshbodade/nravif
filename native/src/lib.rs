#![deny(clippy::all)]

use neon::prelude::*;
use image::ImageReader;
use ravif::{Encoder, Img, ColorSpace};
use std::fs::File;
use std::io::BufReader;
use rgb::{RGB, RGBA};
use neon::types::buffer::TypedArray;

pub fn convert_to_avif(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    // Extract arguments
    let input_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let quality = cx.argument::<JsNumber>(1)?.value(&mut cx) as f32;
    let speed = cx.argument::<JsNumber>(2)?.value(&mut cx) as u8;

    // Open file safely
    let file = File::open(&input_path)
        .or_else(|e| cx.throw_error(format!("Failed to open input file: {}", e)))?;
    let reader = BufReader::new(file);

    // Decode image
    let rgba_image = ImageReader::new(reader)
        .with_guessed_format()
        .or_else(|e| cx.throw_error(format!("Failed to guess image format: {}", e)))?
        .decode()
        .or_else(|e| cx.throw_error(format!("Failed to decode image: {}", e)))?
        .to_rgba8();

    let (width, height) = (rgba_image.width(), rgba_image.height());

    // Convert to RGB pixel array
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for pixel in rgba_image.pixels() {
        let rgba = RGBA::new(pixel[0], pixel[1], pixel[2], pixel[3]);
        pixels.push(RGB::new(rgba.r, rgba.g, rgba.b));
    }

    // Create image buffer for encoding
    let buffer = Img::new(
        pixels.as_slice(),
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    );

    // Encode image
    let encoded_image = Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .with_internal_color_space(ColorSpace::YCbCr)
        .encode_rgb(buffer)
        .or_else(|e| cx.throw_error(format!("Failed to encode image: {}", e)))?;

    // Extract AVIF data
    let avif_data = encoded_image.avif_file;

    // Create JS buffer
    let mut js_buffer = JsBuffer::new(&mut cx, avif_data.len().try_into().unwrap())?;
        js_buffer.as_mut_slice(&mut cx).copy_from_slice(&avif_data);

    Ok(js_buffer) // Correct return type
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("convertToAvif", convert_to_avif)?;
    Ok(())
}
