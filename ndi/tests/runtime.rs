#[test]
fn runtime_loads_and_sender_moves_to_worker_thread() {
    ndi::initialize().expect("NDI runtime should initialize on this CPU");
    let sender = ndi::SendBuilder::new()
        .ndi_name("ndi-rs runtime test".into())
        .clock_audio(false)
        .build()
        .unwrap();
    let source = sender.get_source();
    assert!(source.get_name().contains("ndi-rs runtime test"));
    std::thread::spawn(move || {
        let mut samples = [0.0_f32; 960];
        // SAFETY: initialized, aligned stereo f32 data with 480 samples per
        // channel. The frame is dropped before its stack storage.
        let audio = unsafe {
            ndi::AudioData::from_buffer(48_000, 2, 480, 0, 0, 480 * 4, samples.as_mut_ptr().cast())
        };
        sender.send_audio(&audio);
    })
    .join()
    .unwrap();
    // The Source keeps the native sender alive after the worker exits.
    assert!(source.get_name().contains("ndi-rs runtime test"));
}

#[test]
fn receiver_timeout_does_not_create_frames() {
    ndi::initialize().unwrap();
    let receiver = ndi::Recv::new().unwrap();
    let (mut video, mut audio, mut metadata) = (None, None, None);
    let result = receiver.capture_all(&mut video, &mut audio, &mut metadata, 0);
    let empty = video.is_none() && audio.is_none() && metadata.is_none();
    // Avoid asking the old implementation to free fabricated SDK frames when
    // this regression fails; the passing implementation has nothing to forget.
    if !empty {
        std::mem::forget((video, audio, metadata));
    }
    assert_eq!(result, ndi::FrameType::None);
    assert!(empty, "a timeout must not fabricate SDK-owned frames");

    let (mut video, mut audio, mut metadata) = (None, None, None);
    assert_eq!(receiver.capture_video(&mut video, 0), ndi::FrameType::None);
    assert_eq!(receiver.capture_audio(&mut audio, 0), ndi::FrameType::None);
    assert_eq!(
        receiver.capture_metadata(&mut metadata, 0),
        ndi::FrameType::None
    );
    assert!(video.is_none() && audio.is_none() && metadata.is_none());

    let mut previous_audio = Some(ndi::AudioData::new());
    receiver.capture_audio(&mut previous_audio, 0);
    assert!(
        previous_audio.is_some(),
        "timeouts preserve the previous frame"
    );
}

#[test]
fn sender_timeout_does_not_create_metadata() {
    ndi::initialize().unwrap();
    let sender = ndi::Send::new().unwrap();
    let mut metadata = None;
    assert_eq!(sender.capture(&mut metadata, 0), ndi::FrameType::None);
    assert!(metadata.is_none());
}
