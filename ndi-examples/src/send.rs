use std::iter::FromIterator;

fn main() {
    ndi::initialize().unwrap();

    let send = ndi::SendBuilder::new()
        .ndi_name("MySender".to_string())
        .build()
        .unwrap();

    const NUM_BYTES: usize = 4 * 1920 * 1080;
    let mut buf = vec![0_u8; NUM_BYTES];
    // linear gradient along x
    for (i, byte) in buf.iter_mut().enumerate() {
        let channel = i % 4;
        let pos = i / 4;
        let x = pos % 1920;
        *byte = if channel == 3 {
            255_u8
        } else {
            (x * 255 / 1920) as _
        };
    }

    let video_data_send = ndi::VideoData::from_buffer(
        1920,
        1080,
        ndi::FourCCVideoType::RGBA,
        30,
        1,
        ndi::FrameFormatType::Progressive,
        0,
        4 * 1920,
        None,
        buf.as_mut_slice(),
    );

    println!("Made video data");

    let source = send.get_source();
    let mut recv = ndi::RecvBuilder::new()
        .color_format(ndi::RecvColorFormat::RGBX_RGBA)
        .build()
        .unwrap();
    recv.connect(&source);
    println!("Source: {}", source.get_name());

    let num_connected_to_sender = send.get_no_connections(1000);
    println!("Receivers on sender: {}", num_connected_to_sender);

    let mut video_data = None;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    loop {
        assert!(
            std::time::Instant::now() < deadline,
            "timed out receiving the sent frame"
        );
        send.send_video(&video_data_send);
        let res = recv.capture_video(&mut video_data, 1000);
        if res == ndi::FrameType::Video {
            break;
        }
    }

    let frame = video_data.unwrap();

    println!(
        "Got video data: {}x{} {:?} {} {:?} {:?} {}",
        frame.width(),
        frame.height(),
        frame.four_cc(),
        frame.frame_rate(),
        frame.frame_format_type(),
        frame.data_size_in_bytes(),
        frame.timecode()
    );

    // save result to png, same as save_recv example
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

    buf.save("send.png").unwrap();

    // Runtime cleanup is optional. Let all SDK objects drop before process exit.

    println!("Done");
}
