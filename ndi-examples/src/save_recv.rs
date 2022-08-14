extern crate ndi;
use std::iter::FromIterator;

fn main() {
    ndi::initialize().unwrap();

    let find = ndi::FindBuilder::new().build().unwrap();
    let sources = find.current_sources(1000).unwrap();

    let mut recv = ndi::RecvBuilder::new()
        .color_format(ndi::RecvColorFormat::RGBX_RGBA)
        .build()
        .unwrap();
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
        let size = frame.height() * frame.line_stride_in_bytes().unwrap();
        std::slice::from_raw_parts(frame.p_data(), size as _)
    };
    let frame_vec = Vec::from_iter(frame_vec.to_owned());
    let buf = image::ImageBuffer::<image::Rgba<u8>, Vec<_>>::from_vec(
        frame.width(),
        frame.height(),
        frame_vec,
    )
    .ok_or("Failed to create image")
    .unwrap();

    buf.save("save_recv.png").unwrap();

    unsafe {
        ndi::cleanup();
    }

    println!("Done");
}
