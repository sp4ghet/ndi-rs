use ndi::AudioData;

#[test]
fn caller_owned_planar_audio_preserves_settings_and_storage() {
    let mut samples = [0.25_f32, -0.25, 0.5, -0.5];
    let pointer = samples.as_mut_ptr().cast::<u8>();
    // SAFETY: two channels of two initialized f32 samples, with an 8-byte
    // channel stride. The storage outlives the frame and is not mutated.
    let frame = unsafe { AudioData::from_buffer(48_000, 2, 2, 123, 456, 8, pointer) };
    assert_eq!(frame.sample_rate(), 48_000);
    assert_eq!(frame.no_channels(), 2);
    assert_eq!(frame.no_samples(), 2);
    assert_eq!(frame.channel_stride_in_bytes(), 8);
    assert_eq!(frame.timecode(), 123);
    assert_eq!(frame.timestamp(), Some(456));
    assert_eq!(frame.p_data(), pointer);
    assert!(matches!(frame.four_cc(), ndi::FourCCAudioType::FLTP));
    assert_eq!(frame.metadata(), "");
    drop(frame);
    samples[0] = 1.0;
    assert_eq!(samples, [1.0, -0.25, 0.5, -0.5]);
}

#[test]
fn empty_audio_has_no_metadata() {
    assert_eq!(AudioData::new().metadata(), "");
}

#[test]
fn sender_can_move_between_threads() {
    fn assert_send<T: std::marker::Send>() {}
    assert_send::<ndi::Send>();
}
