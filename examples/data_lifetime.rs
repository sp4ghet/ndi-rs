use std::thread;

fn get_frame(source: &ndi::Source) -> Result<ndi::VideoData, String> {
    let mut recv = ndi::RecvBuilder::new().build()?;
    recv.connect(source);

    let mut video_data = None;
    loop {
        let response = recv.capture_video(&mut video_data, 1000);
        if response == ndi::FrameType::Video {
            break;
        }
    }

    video_data.ok_or("No video".to_string())
}

fn main() -> Result<(), String> {
    ndi::initialize()?;

    let find = ndi::FindBuilder::new().build()?;
    let sources = find.current_sources(1000)?;

    let frame = get_frame(&sources[0])?;

    thread::sleep(std::time::Duration::from_millis(1000));

    println!("Frame received: {}x{}", frame.xres(), frame.yres());

    Ok(())
}
