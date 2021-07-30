use std::thread;

fn get_frame(source: &ndi::Source) -> ndi::VideoData {
    let mut recv = ndi::RecvBuilder::new().build().unwrap();
    recv.connect(source);

    let mut video_data = None;
    loop {
        let response = recv.capture_video(&mut video_data, 1000);
        if response == ndi::FrameType::Video {
            break;
        }
    }

    video_data.expect("no video data")
}

fn main() {
    ndi::initialize().unwrap();

    let find = ndi::FindBuilder::new().build().unwrap();
    let sources = find.current_sources(1000).unwrap();

    let frame = get_frame(&sources[0]);

    thread::sleep(std::time::Duration::from_millis(1000));

    println!("Frame received: {}x{}", frame.width(), frame.height());
}
