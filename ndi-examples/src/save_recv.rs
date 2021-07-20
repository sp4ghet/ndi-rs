#[macro_use]
extern crate error_chain;
extern crate ndi;
use std::iter::FromIterator;

mod errors {

    error_chain! {
        foreign_links {
            NDI(ndi::NDIError);
            Utf8(std::str::Utf8Error);
            Image(image::error::ImageError);
        }
    }
}
use errors::*;

fn main() -> Result<()> {
    ndi::initialize()?;

    let find = ndi::FindBuilder::new().build()?;
    let sources = find.current_sources(1000)?;

    let mut recv = ndi::RecvBuilder::new()
        .color_format(ndi::RecvColorFormat::RGBX_RGBA)
        .build()?;
    recv.connect(&sources[0]);

    let mut video_data = None;
    loop {
        let res = recv.capture_video(&mut video_data, 1000);
        if res == ndi::FrameType::Video {
            break;
        }
    }

    let frame = video_data.unwrap();

    let frame_vec = unsafe {
        assert!(!frame.p_data().is_null());
        let size = frame.yres() * frame.line_stride_in_bytes().unwrap();
        std::slice::from_raw_parts(frame.p_data(), size as _)
    };
    let frame_vec = Vec::from_iter(frame_vec.to_owned());
    let buf = image::ImageBuffer::<image::Rgba<u8>, Vec<_>>::from_vec(
        frame.xres(),
        frame.yres(),
        frame_vec,
    )
    .ok_or("Failed to create image")?;

    buf.save("save_recv.png")?;
    println!("Done");

    Ok(())
}
