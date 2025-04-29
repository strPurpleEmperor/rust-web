use std::io::Cursor;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_multipart::form::text::Text;
use actix_web::{post, Error, HttpResponse};
#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    t_type: Text<String>,
}

#[post("/convert")]
pub async fn convert_heic(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, Error> {
    // 获取HEIC文件数据
    let heic_data = get_file_bytes(&form.file)?;
    // 解码HEIC并转换为JPEG
    let jpeg_data = decode_heic_to_jpeg(&heic_data, form.t_type.clone())?;
    Ok(HttpResponse::Ok()
        .content_type(format!("image/{}", form.t_type.as_str()))
        .body(jpeg_data))
}

use image::{DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgb, Rgba};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

fn decode_heic_to_jpeg(heic_data: &[u8], f_type:String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(heic_data)?;
    let handle = ctx.primary_image_handle()?;
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let planes = image.planes();
    let rgb_plane = planes.interleaved.unwrap();
    let width = image.width();
    let height = image.height();

    // 计算通道数
    let bytes_per_pixel = rgb_plane.stride / width as usize;
    let channels = match bytes_per_pixel {
        3 => 3,
        4 => 4,
        _ => return Err("Unsupported pixel format".into()),
    };

    // 去除行填充并提取有效数据
    let mut raw_data = Vec::with_capacity(width as usize * height as usize * channels);
    for row in 0..height as usize {
        let start = row * rgb_plane.stride;
        let end = start + width as usize * channels;
        raw_data.extend_from_slice(&rgb_plane.data[start..end]);
    }
    let  t_type :ImageFormat;
    match f_type.as_str() {
        "jpg"=>t_type = ImageFormat::Jpeg,
        "png"=>t_type = ImageFormat::Png,
        "webp"=>t_type = ImageFormat::WebP,
        _ => t_type = ImageFormat::Jpeg
    }
    // 动态构建 ImageBuffer
    let jpeg_data = match channels {
        3 => {
            let img_buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, raw_data)
                .ok_or("Invalid RGB data")?;
            encode_to_jpeg(img_buffer, t_type)
        },
        4 => {
            let img_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_data)
                .ok_or("Invalid RGBA data")?;
            let dynamic_img = DynamicImage::ImageRgba8(img_buffer);
            let rgb_img = dynamic_img.to_rgb8();
            encode_to_jpeg(rgb_img, t_type)
        },
        _ => unreachable!(),
    }?;

    Ok(jpeg_data)
}
fn encode_to_jpeg(img: ImageBuffer<Rgb<u8>, Vec<u8>>, t_type: ImageFormat) -> Result<Vec<u8>, ImageError> {
    let mut jpeg_data = Vec::new();
    img.write_to(&mut Cursor::new(&mut jpeg_data), t_type)?;
    Ok(jpeg_data)
}
fn get_file_bytes(temp_file: &TempFile) -> Result<Vec<u8>, std::io::Error> {
    // 获取临时文件路径
    let file_path = temp_file.file.path();
    // 读取文件内容到 u8 数组
    std::fs::read(file_path)
}
