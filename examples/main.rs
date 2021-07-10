use std::io;
use std::time::Instant;

fn run() -> Result<(), String> {
    ndi::initialize()?;

    let find = ndi::Find::new()?;
    println!("Looking for sources");
    let sources = find.current_sources(1000)?;

    if sources.len() == 0 {
        return Err("No sources found".to_string());
    }

    println!("Select device:");
    for (i, source) in sources.iter().enumerate() {
        println!("  {}: {}", i, source.get_name()?);
    }

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).map_err(|e| e.to_string())?;
    let i = buf.trim_end().parse::<usize>().map_err(|e| e.to_string())?;

    let recv_builder = ndi::RecvBuilder::new()
        .color_format(ndi::RecvColorFormat::RGBX_RGBA)
        .ndi_recv_name("ndi-rs".to_string());
    let mut recv = recv_builder.build()?;
    recv.connect(&sources[i]);

    let name = sources[i].get_name().unwrap();
    println!("Connected to NDI device {}", name);

    let start = Instant::now();
    while Instant::now().duration_since(start).as_secs() < 5 {
        let mut video_data = None;
        let mut audio_data = None;
        let mut meta_data = None;
        let response = recv.capture(&mut video_data, &mut audio_data, &mut meta_data, 1000);
        let video_data = video_data.ok_or("Failed to get video data from capture".to_string())?;
        let audio_data = audio_data.ok_or("Failed to get audio data from capture".to_string())?;
        let meta_data = meta_data.ok_or("Failed to get meta data from capture".to_string())?;

        let (total, dropped) = recv.get_performance();
        println!("total:\n {}dropped:\n {}", total, dropped);

        match response {
            ndi::FrameType::None => println!("Nothing"),
            ndi::FrameType::Video => {
                println!(
                    "Got video data: {}x{} {:?}",
                    video_data.xres(),
                    video_data.yres(),
                    video_data.four_cc()
                );
                recv.free_video_data(video_data);
            }
            ndi::FrameType::Audio => {
                println!(
                    "Got audio data. Channels: {}, Samples: {}, Stride: {}",
                    audio_data.no_channels(),
                    audio_data.no_samples(),
                    audio_data.channel_stride_in_bytes()
                );
                recv.free_audio_data(audio_data);
            }
            ndi::FrameType::StatusChange => {
                println!("Status change.")
            }
            ndi::FrameType::ErrorFrame => {
                println!("Error")
            }
            ndi::FrameType::Metadata => {
                println!("Got metadata. {:?}", meta_data.length())
            }
        }
    }

    println!("Done");

    ndi::cleanup();

    Ok(())
}

fn main() {
    run().unwrap();
}
