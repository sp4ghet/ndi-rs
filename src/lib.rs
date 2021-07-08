use core::panic;
use std::ffi::CStr;
use std::fmt::Display;
use std::mem;
use std::time::Instant;

pub mod internal;
// pub mod recv;

use internal::bindings::*;

const NULL: usize = 0;

#[repr(i32)]
pub enum FrameType {
    None = NDIlib_frame_type_e_NDIlib_frame_type_none,
    Video = NDIlib_frame_type_e_NDIlib_frame_type_video,
    Audio = NDIlib_frame_type_e_NDIlib_frame_type_audio,
    StatusChange = NDIlib_frame_type_e_NDIlib_frame_type_status_change,
    Error = NDIlib_frame_type_e_NDIlib_frame_type_error,
    Metadata = NDIlib_frame_type_e_NDIlib_frame_type_metadata,
}

#[repr(i32)]
pub enum FrameFormatType {
    Progressive = NDIlib_frame_format_type_e_NDIlib_frame_format_type_progressive,
    Interleaved = NDIlib_frame_format_type_e_NDIlib_frame_format_type_interleaved,
    Field0 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_0,
    Field1 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_1,
}

#[repr(i32)]
pub enum FourCCVideoType {
    UYVY = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVY,
    UYVA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVA,
    P216 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_P216,
    PA16 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_PA16,
    YV12 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_YV12,
    I420 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_I420,
    NV12 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_NV12,
    BGRA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRA,
    RGBA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBA,
    BGRX = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRX,
    RGBX = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBX,
}

#[repr(i32)]
pub enum FourCCAudioType {
    FLTP = NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP,
}

pub struct Source {
    p_instance: NDIlib_source_t,
}

impl Source {
    fn from_ptr(ptr: *const NDIlib_source_t) -> Self {
        // TODO: error check somehow and return Err
        Self {
            p_instance: unsafe { *ptr },
        }
    }

    pub fn get_name(&self) -> Result<String, String> {
        let name_char_ptr = self.p_instance.p_ndi_name as _;
        let name = unsafe {
            CStr::from_ptr(name_char_ptr)
                .to_owned()
                .to_str()
                .map_err(|e| e.to_string())?
                .to_string()
        };
        Ok(name)
    }
}

pub struct VideoData {
    p_instance: mem::MaybeUninit<NDIlib_video_frame_v2_t>,
}

impl VideoData {
    pub fn new() -> Self {
        let p_instance: mem::MaybeUninit<NDIlib_video_frame_v2_t> = mem::MaybeUninit::uninit();
        Self { p_instance }
    }
}

pub struct AudioData {
    p_instance: mem::MaybeUninit<NDIlib_audio_frame_v2_t>,
}

impl AudioData {
    pub fn new() -> Self {
        let p_instance: mem::MaybeUninit<NDIlib_audio_frame_v2_t> = mem::MaybeUninit::uninit();

        Self { p_instance }
    }
}

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct RecvPerformance {
    pub video_frames: i64,
    pub audio_frames: i64,
    pub metadata_frames: i64,
}

impl RecvPerformance {
    fn from_binding(perf: NDIlib_recv_performance_t) -> Self {
        Self {
            video_frames: perf.video_frames,
            audio_frames: perf.audio_frames,
            metadata_frames: perf.metadata_frames,
        }
    }
}

impl Display for RecvPerformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Video frames: {}", self.video_frames)?;
        writeln!(f, "Audio frames: {}", self.audio_frames)?;
        writeln!(f, "Metadata frames: {}", self.metadata_frames)
    }
}

pub struct Recv {
    p_instance: NDIlib_recv_instance_t,
}

impl Recv {
    pub fn new() -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_recv_create_v3(NULL as _) };

        if p_instance.is_null() {
            return Err("Failed to create NDI Recv instance".to_string());
        }

        Ok(Self { p_instance })
    }

    pub fn connect(&self, source: &Source) {
        let instance: *const NDIlib_source_t = &source.p_instance;
        unsafe { NDIlib_recv_connect(self.p_instance, instance) };
    }

    pub fn capture(
        &self,
        video_data: &mut VideoData,
        audio_data: &mut AudioData,
        timeout_ms: u32,
    ) -> FrameType {
        let response = unsafe {
            NDIlib_recv_capture_v2(
                self.p_instance,
                video_data.p_instance.as_mut_ptr(),
                audio_data.p_instance.as_mut_ptr(),
                NULL as _,
                timeout_ms,
            )
        };

        #[allow(non_upper_case_globals)]
        match response {
            NDIlib_frame_type_e_NDIlib_frame_type_none => FrameType::None,
            NDIlib_frame_type_e_NDIlib_frame_type_video => FrameType::Video,
            NDIlib_frame_type_e_NDIlib_frame_type_audio => FrameType::Audio,
            NDIlib_frame_type_e_NDIlib_frame_type_status_change => FrameType::StatusChange,
            NDIlib_frame_type_e_NDIlib_frame_type_error => FrameType::Error,
            NDIlib_frame_type_e_NDIlib_frame_type_metadata => FrameType::Metadata,
            x => panic!("Unknown frame type {} encountered", x),
        }
    }

    pub fn get_performance(&self) -> (RecvPerformance, RecvPerformance) {
        let mut p_total: mem::MaybeUninit<NDIlib_recv_performance_t> = mem::MaybeUninit::uninit();
        let mut p_dropped: mem::MaybeUninit<NDIlib_recv_performance_t> = mem::MaybeUninit::uninit();
        unsafe {
            NDIlib_recv_get_performance(
                self.p_instance,
                p_total.as_mut_ptr(),
                p_dropped.as_mut_ptr(),
            )
        };

        let total_perf = RecvPerformance::from_binding(unsafe { p_total.assume_init() });
        let dropped_perf = RecvPerformance::from_binding(unsafe { p_dropped.assume_init() });

        (total_perf, dropped_perf)
    }

    pub fn free_video_data(&self, video_data: VideoData) {
        unsafe {
            NDIlib_recv_free_video_v2(self.p_instance, video_data.p_instance.as_ptr());
        }
    }

    pub fn free_audio_data(&self, audio_data: AudioData) {
        unsafe {
            NDIlib_recv_free_audio_v2(self.p_instance, audio_data.p_instance.as_ptr());
        }
    }
}

// impl Drop for Recv {
//     fn drop(&mut self) {
//         unsafe { NDIlib_recv_destroy(self.p_instance) };
//     }
// }

pub struct Find {
    p_instance: NDIlib_find_instance_t,
}

impl Find {
    pub fn new() -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_find_create_v2(NULL as _) };
        if p_instance.is_null() {
            return Err("Failed to create new NDI Find instance.".to_string());
        };

        Ok(Self { p_instance })
    }

    pub fn current_sources(&self) -> Result<Vec<Source>, String> {
        let mut no_sources = 0;
        let mut p_sources: *const NDIlib_source_t = NULL as _;
        let start = Instant::now();
        while no_sources == 0 {
            // timeout if it takes an unreasonable amount of time
            if Instant::now().duration_since(start).as_secs() > 10 {
                return Err("Timeout on finding NDI sources".to_string());
            }

            p_sources =
                unsafe { NDIlib_find_get_current_sources(self.p_instance, &mut no_sources) };
        }

        let mut sources: Vec<Source> = vec![];
        for _ in 0..no_sources {
            sources.push(Source::from_ptr(p_sources));
            p_sources = unsafe { p_sources.add(1) };
        }

        Ok(sources)
    }
}

// // TODO: Rust seems to have issues with calling find_destroy and
// // you get a STATUS_HEAP_CORRUPTION when used
// impl Drop for Find {
//     fn drop(&mut self) {
//         unsafe { NDIlib_find_destroy(self.p_instance) };
//     }
// }

pub fn initialize() -> Result<(), String> {
    if !unsafe { NDIlib_initialize() } {
        return Err("Failed to initialize NDIlib".to_string());
    };

    Ok(())
}

pub fn cleanup() {
    unsafe { NDIlib_destroy() };
}
