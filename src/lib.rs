#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::time::Instant;

mod bindings;

pub fn hoge() {
    unsafe {
        if !bindings::NDIlib_initialize() {
            return;
        };

        let ndi_find = bindings::NDIlib_find_create_v2(0 as _);
        if ndi_find.is_null() {
            return;
        }
        let mut no_sources = 0;
        let mut p_sources: *const bindings::NDIlib_source_t = 0 as _;
        while no_sources == 0 {
            println!("Looking for sources");
            bindings::NDIlib_find_wait_for_sources(ndi_find, 1000);
            p_sources = bindings::NDIlib_find_get_current_sources(ndi_find, &mut no_sources);
        }
        println!("number of sources: {}", no_sources);
        p_sources = p_sources.add((no_sources as usize) - 1);
        let name = (*p_sources).p_ndi_name as *mut i8;
        let name = CString::from_raw(name);
        println!(
            "Connected to NDI channel: {:?}",
            name.to_str().unwrap().to_owned()
        );

        let ndi_recv = bindings::NDIlib_recv_create_v3(0 as _);
        if ndi_recv.is_null() {
            return;
        }

        bindings::NDIlib_recv_connect(ndi_recv, p_sources);
        bindings::NDIlib_find_destroy(ndi_find);

        let start = Instant::now();
        while Instant::now()
            .checked_duration_since(start)
            .map_or(true, |x| x.as_secs() < 10)
        {
            let p_video_data = 0 as _;
            let p_audio_data = 0 as _;
            match bindings::NDIlib_recv_capture_v3(
                ndi_recv,
                p_video_data,
                p_audio_data,
                0 as _,
                1000,
            ) {
                bindings::NDIlib_frame_type_e_NDIlib_frame_type_none => {
                    println!("No data received");
                }
                bindings::NDIlib_frame_type_e_NDIlib_frame_type_video => {
                    println!(
                        "Video data received ({} x {})",
                        (*p_video_data).xres,
                        (*p_video_data).yres
                    );
                    bindings::NDIlib_recv_free_video_v2(ndi_recv, p_video_data);
                }
                bindings::NDIlib_frame_type_e_NDIlib_frame_type_audio => {
                    println!(
                        "Audio data received: {} samples",
                        (*p_audio_data).no_samples
                    );
                    bindings::NDIlib_recv_free_audio_v3(ndi_recv, p_audio_data);
                }
                bindings::NDIlib_frame_type_e_NDIlib_frame_type_error => {
                    println!("NDIlib_frame_type_error");
                }
                bindings::NDIlib_frame_type_e_NDIlib_frame_type_status_change => {
                    println!("Status change");
                }
                x => {
                    println!("Something else {}", x);
                }
            };
        }

        println!("Done");
        bindings::NDIlib_recv_destroy(ndi_recv);
        bindings::NDIlib_destroy();
    }
}
