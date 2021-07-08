use std::io;
use std::time::Instant;

fn run() -> Result<(), String> {
    ndi::initialize()?;

    let find = ndi::Find::new()?;
    println!("Looking for sources");
    let sources = find.current_sources()?;

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

    let recv = ndi::Recv::new()?;
    recv.connect(&sources[i]);

    let name = sources[i].get_name().unwrap();
    println!("Connected to NDI device {}", name);

    let start = Instant::now();
    while Instant::now().duration_since(start).as_secs() < 3 {
        let mut video_data = ndi::VideoData::new();
        let mut audio_data = ndi::AudioData::new();

        let response = recv.capture(&mut video_data, &mut audio_data, 1000);

        // let (total, dropped) = recv.get_performance();
        // println!("total:\n {}dropped:\n {}", total, dropped);

        match response {
            ndi::FrameType::None => println!("Nothing"),
            ndi::FrameType::Video => {
                println!("Got video data.");
                recv.free_video_data(video_data);
            }
            ndi::FrameType::Audio => {
                println!("Got audio data.");
                recv.free_audio_data(audio_data);
            }
            ndi::FrameType::StatusChange => {
                println!("Status change.")
            }
            ndi::FrameType::Error => {
                println!("Error")
            }
            ndi::FrameType::Metadata => {
                println!("Got metadata.")
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
