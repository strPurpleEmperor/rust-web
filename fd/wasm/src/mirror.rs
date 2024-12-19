use wasm_bindgen::{JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use std::io::Cursor;
use image::ImageFormat::Png;
use crate::mirror_gif::mirror_image;

#[wasm_bindgen]
#[derive(Debug)]
#[derive(Clone)]
pub enum MirrorDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[wasm_bindgen]
pub fn mirror(image_data: &[u8], direction: MirrorDirection) -> Result<Vec<u8>, JsValue> {
    // 加载图片
    let img = match image::load_from_memory(image_data) {
        Ok(img) => img.to_rgba8(),
        Err(_) => return Err(JsValue::from("Failed to decode image")),
    };
    // 镜像操作
    let output_img = mirror_image(img, direction);


    // 编码为png
    let mut output_data = Cursor::new(Vec::new());
    if let Err(_) = output_img.write_to(&mut output_data, Png) {
        return Err(JsValue::from("Failed to encode mirrored image"));
    }

    Ok(output_data.into_inner())
}

