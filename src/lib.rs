use wasm_bindgen::prelude::wasm_bindgen;

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, color_thief::ColorFormat) {
    match img {
        // without alpha
        image::DynamicImage::ImageRgb8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgb)
        }
        // with alpha
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}

#[wasm_bindgen]
pub fn get_colors(img_buff: Vec<u8>, format: &str)->Vec<u8>{
    let fmt = image::ImageFormat::from_extension(format).unwrap();
    let img = image::load_from_memory_with_format(&img_buff, fmt).unwrap();
    let (buffer, color_type) = get_image_buffer(img);
    
    let colors = color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();
    let mut ans:Vec<u8> = vec![] ;
    for color in colors {
            let arr = [color.r, color.g, color.b];
            ans.extend_from_slice(&arr);
    }
    ans
    // colors
}


// fn main() {
//     let buf = read("./src/Designer.jpeg").unwrap();
    
//     // println!("hello")
// }
 
