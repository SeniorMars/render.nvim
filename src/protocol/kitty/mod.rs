// this code is inspired by [viuer](https://github.com/atanunq/viuer)
// and [image.nvim](https://github.com/3rd/image.nvim/tree/master/lua/image/backends/kitty)
mod control;
mod helpers;

use crate::error::Error;
use super::Protocol;

// const TEMP_FILE_PREFIX: &str = ".tty-graphics-protocol.render.nvim";
pub struct Kitty {}

// impl Kitty {
//
// }

// impl Protocol for Kitty {
//     fn render(data: &[u8]) -> Result<(), Error> {
//
//     }
// }

// fn print_remote() -> (u32, u32) {
//     let rgba = img.to_rgba8();
//     let raw = rgba.as_raw();
//     let encoded = base64::encode(raw);
//     let mut iter = encoded.chars().peekable();
//
//     adjust_offset(stdout, config)?;
//
//     let (w, h) = find_best_fit(img, config.width, config.height);
//
//     let first_chunk: String = iter.by_ref().take(4096).collect();
//
//     // write the first chunk, which describes the image
//     write!(
//         stdout,
//         "\x1b_Gf=32,a=T,t=d,s={},v={},c={},r={},m=1;{}\x1b\\",
//         img.width(),
//         img.height(),
//         w,
//         h,
//         first_chunk
//     )?;
//
//     // write all the chunks, each containing 4096 bytes of data
//     while iter.peek().is_some() {
//         let chunk: String = iter.by_ref().take(4096).collect();
//         let m = if iter.peek().is_some() { 1 } else { 0 };
//         write!(stdout, "\x1b_Gm={};{}\x1b\\", m, chunk)?;
//     }
//     writeln!(stdout)?;
//     stdout.flush()?;
//     // Ok((w, h))
//     return (0, 0)
// }

// #[test]
// fn kitty() {
//     use image::{DynamicImage, GenericImage};
//     let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(1, 2));
//     img.put_pixel(0, 1, image::Rgba([2, 4, 6, 8]));
//
//     let mut vec = Vec::new();
//     // assert_eq!(print_remote(&mut vec, &img, &config).unwrap(), (1, 1));
//     let result = std::str::from_utf8(&vec).unwrap();
//
//     assert_eq!(
//         result,
//         "\x1b[6;3H\x1b_Gf=32,a=T,t=d,s=1,v=2,c=1,r=1,m=1;AAAAAAIEBgg=\x1b\\\n"
//     );
// }
