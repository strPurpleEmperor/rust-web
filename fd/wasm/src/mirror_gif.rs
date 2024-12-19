use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use image::{ RgbaImage, Frame, AnimationDecoder, Delay};
use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use std::io::Cursor;
use crate::mirror::MirrorDirection;
#[wasm_bindgen]
pub fn mirror_gif(input_data: Uint8Array, direction: MirrorDirection) -> Result<Uint8Array, JsValue> {
    // 解码输入的GIF文件
    let input_vec = input_data.to_vec();
    let cursor = Cursor::new(input_vec);

    // 使用 GifDecoder 来解码 GIF 幘并提取帧和延迟信息
    let decoder = GifDecoder::new(cursor).map_err(|_| JsValue::from_str("Failed to decode GIF"))?;
    let frames = decoder.into_frames().collect::<Result<Vec<_>, _>>().map_err(|_| JsValue::from_str("Failed to collect GIF frames"))?;
    let mut mirrored_frames: Vec<(RgbaImage, Delay)> = vec![] ;
    for frame in frames {
        let delay = frame.clone().delay(); // 获取原始帧延迟
        let image = frame.clone().into_buffer(); // 获取 RGBA 图像数据并克隆
        let new_image = mirror_image(image, direction.clone());
        mirrored_frames.push((new_image, delay))
    }


    // 编码所有帧为新的GIF
    let mut output_data = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut output_data);
        encoder.set_repeat(Repeat::Infinite).expect("TODO: panic message");
        // 处理每一帧
        for (frame, delay) in mirrored_frames {
            // 创建 Frame 并设置延迟
            let gif_frame = Frame::from_parts(frame, 0, 0, delay);
            // 编码帧
            encoder.encode_frame(gif_frame).map_err(|_| JsValue::from_str("Failed to encode GIF"))?;
        }
    }

    // 返回镜像后的GIF二进制数据
    Ok(Uint8Array::from(&output_data[..]))
}

pub fn mirror_image(mut image: RgbaImage, direction: MirrorDirection) -> RgbaImage {
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
        },
        MirrorDirection::RightToLeft => {
            for y in 0..height {
                for x in 0..(width / 2) {
                    let right_pixel = image.get_pixel(width - x - 1, y);
                    let left_x = x;
                    image.put_pixel(left_x, y, right_pixel.clone());
                }
            }
        },
        MirrorDirection::TopToBottom => {
            for x in 0..width {
                for y in 0..(height / 2) {
                    let top_pixel = image.get_pixel(x, y);
                    let bottom_y = height - y - 1;
                    image.put_pixel(x, bottom_y, top_pixel.clone());
                }
            }
        },
        MirrorDirection::BottomToTop => {
            for x in 0..width {
                for y in 0..(height / 2) {
                    let bottom_pixel = image.get_pixel(x, height - y - 1);
                    let top_y = y;
                    image.put_pixel(x, top_y, bottom_pixel.clone());
                }
            }
        },
    }

    image
}
