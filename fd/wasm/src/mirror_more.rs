use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::ImageFormat::Png;
use image::ImageReader;
use image::{AnimationDecoder, Delay, DynamicImage, Frame, ImageFormat, RgbaImage};
use js_sys::{Promise, Uint8Array};
use std::io::{BufReader, Cursor};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum MirrorDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}
#[wasm_bindgen]
pub fn mirror_image_async(input_data: Uint8Array, direction: MirrorDirection) -> Promise {
    let input_vec = input_data.to_vec();

    // 将同步操作转换为异步并返回 Promise
    future_to_promise(async move {
        let result = mirror_image_process(input_vec, direction)
            .map_err(|err| JsValue::from_str(&format!("Error processing image: {:?}", err)))?;
        Ok(JsValue::from(result)) // 将 Uint8Array 包装为 JsValue
    })
}

fn mirror_image_process(
    input_vec: Vec<u8>,
    direction: MirrorDirection,
) -> Result<Uint8Array, JsValue> {
    // 尝试解析图像格式
    let cursor = Cursor::new(input_vec.clone());
    let format = ImageReader::new(BufReader::new(cursor))
        .with_guessed_format()
        .map_err(|_| JsValue::from_str("无法识别图像格式"))?
        .format()
        .ok_or_else(|| JsValue::from_str("未知的图像格式"))?;

    match format {
        ImageFormat::Gif => mirror_gif(input_vec, direction), // 处理 GIF
        _ => mirror_static_image(input_vec, direction, format), // 处理静态图片
    }
}

fn mirror_gif(input_vec: Vec<u8>, direction: MirrorDirection) -> Result<Uint8Array, JsValue> {
    let cursor = Cursor::new(input_vec);

    // 解码 GIF
    let decoder = GifDecoder::new(cursor).map_err(|_| JsValue::from_str("GIF 解码失败"))?;
    let frames = decoder
        .into_frames()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| JsValue::from_str("无法收集 GIF 帧"))?;

    let mut mirrored_frames: Vec<(RgbaImage, Delay)> = vec![];
    for frame in frames {
        let delay = frame.delay();
        let image = frame.into_buffer();
        let new_image = mirror_rgba_image(image, direction.clone());
        mirrored_frames.push((new_image, delay));
    }

    // 编码为新的 GIF
    let mut output_data = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut output_data);
        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|_| JsValue::from_str("无法设置 GIF 循环"))?;
        for (frame, delay) in mirrored_frames {
            let gif_frame = Frame::from_parts(frame, 0, 0, delay);
            encoder
                .encode_frame(gif_frame)
                .map_err(|_| JsValue::from_str("无法编码 GIF 帧"))?;
        }
    }

    Ok(Uint8Array::from(&output_data[..]))
}

fn mirror_static_image(
    input_vec: Vec<u8>,
    direction: MirrorDirection,
    format: ImageFormat,
) -> Result<Uint8Array, JsValue> {
    let cursor = Cursor::new(input_vec);

    // 解码静态图像
    let image = image::load(cursor, format).map_err(|_| JsValue::from_str("图像解码失败"))?;
    let rgba_image = image.to_rgba8();

    // 镜像处理
    let mirrored_image = mirror_rgba_image(rgba_image, direction);

    // 编码回原格式
    let mut output_data = Vec::new();
    DynamicImage::ImageRgba8(mirrored_image)
        .write_to(&mut Cursor::new(&mut output_data), Png)
        .map_err(|_| JsValue::from_str("图像编码失败"))?;

    Ok(Uint8Array::from(&output_data[..]))
}

fn mirror_rgba_image(mut image: RgbaImage, direction: MirrorDirection) -> RgbaImage {
    let (width, height) = image.dimensions();

    match direction {
        MirrorDirection::LeftToRight => {
            for y in 0..height {
                for x in 0..(width / 2) {
                    let left_pixel = image.get_pixel(x, y);
                    let right_x = width - x - 1;
                    image.put_pixel(right_x, y, left_pixel.clone());
                }
            }
        }
        MirrorDirection::RightToLeft => {
            for y in 0..height {
                for x in 0..(width / 2) {
                    let right_pixel = image.get_pixel(width - x - 1, y);
                    let left_x = x;
                    image.put_pixel(left_x, y, right_pixel.clone());
                }
            }
        }
        MirrorDirection::TopToBottom => {
            for x in 0..width {
                for y in 0..(height / 2) {
                    let top_pixel = image.get_pixel(x, y);
                    let bottom_y = height - y - 1;
                    image.put_pixel(x, bottom_y, top_pixel.clone());
                }
            }
        }
        MirrorDirection::BottomToTop => {
            for x in 0..width {
                for y in 0..(height / 2) {
                    let bottom_pixel = image.get_pixel(x, height - y - 1);
                    let top_y = y;
                    image.put_pixel(x, top_y, bottom_pixel.clone());
                }
            }
        }
    }

    image
}
