#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::mem;
use std::time::Instant;

mod bindings;
use bindings::*;

const NULL: usize = 0;

pub fn hoge() {
    unsafe {
        if !bindings::NDIlib_initialize() {
            return;
        };

        let ndi_find: NDIlib_find_instance_t = NDIlib_find_create_v2(NULL as _);
        if ndi_find.is_null() {
            return;
        }
        let mut no_sources = 0;
        let mut p_sources: *const NDIlib_source_t = NULL as _;
        while no_sources == 0 {
            println!("Looking for sources");
            NDIlib_find_wait_for_sources(ndi_find, 1000);
            p_sources = NDIlib_find_get_current_sources(ndi_find, &mut no_sources);
        }

        let ndi_recv: NDIlib_recv_instance_t = NDIlib_recv_create_v3(NULL as _);
        if ndi_recv.is_null() {
            return;
        }

        NDIlib_recv_connect(ndi_recv, p_sources);
        NDIlib_find_destroy(ndi_find);

        let start = Instant::now();
        while Instant::now()
            .checked_duration_since(start)
            .map_or(true, |x| x.as_secs() < 30)
        {
            let mut p_video_data: mem::MaybeUninit<NDIlib_video_frame_v2_t> =
                mem::MaybeUninit::uninit();
            let mut p_audio_data: mem::MaybeUninit<NDIlib_audio_frame_v2_t> =
                mem::MaybeUninit::uninit();
            let response = NDIlib_recv_capture_v2(
                ndi_recv,
                p_video_data.as_mut_ptr(),
                p_audio_data.as_mut_ptr(),
                0 as _,
                1000,
            );
            let p_total = NULL as _;
            let p_dropped = NULL as _;
            NDIlib_recv_get_performance(ndi_recv, p_total, p_dropped);
            println!("total: {:?}  dropped: {:?}", p_total, p_dropped);
            match response {
                NDIlib_frame_type_e_NDIlib_frame_type_none => {
                    println!("No data received");
                }
                NDIlib_frame_type_e_NDIlib_frame_type_video => {
                    if p_video_data.as_ptr().is_null() {
                        continue;
                    }
                    let video_data = p_video_data.assume_init();
                    println!(
                        "Video data received: ({} x {}).",
                        video_data.xres, video_data.yres
                    );
                    NDIlib_recv_free_video_v2(ndi_recv, p_video_data.as_mut_ptr());
                }
                NDIlib_frame_type_e_NDIlib_frame_type_audio => {
                    if p_audio_data.as_ptr().is_null() {
                        continue;
                    }
                    println!(
                        "Audio data received: {} samples",
                        p_audio_data.assume_init().no_samples
                    );
                    NDIlib_recv_free_audio_v2(ndi_recv, p_audio_data.as_mut_ptr());
                }
                NDIlib_frame_type_e_NDIlib_frame_type_error => {
                    println!("NDIlib_frame_type_error");
                }
                NDIlib_frame_type_e_NDIlib_frame_type_status_change => {
                    println!("Status change");
                }
                x => {
                    println!("Something else {}", x);
                }
            };
        }

        println!("Done");

        NDIlib_recv_destroy(ndi_recv);
        NDIlib_destroy();
    }
}
