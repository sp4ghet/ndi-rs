use super::internal::bindings::*;
use super::*;
use core::panic;
use std::fmt::Display;
use std::mem;

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
            let mut video = if let Some(x) = video_data.p_instance {
                mem::MaybeUninit::new(x)
            } else {
                mem::MaybeUninit::uninit()
            };
            let mut audio = if let Some(x) = audio_data.p_instance {
                mem::MaybeUninit::new(x)
            } else {
                mem::MaybeUninit::uninit()
            };
            let response = NDIlib_recv_capture_v2(
                self.p_instance,
                video.as_mut_ptr(),
                audio.as_mut_ptr(),
                NULL as _,
                timeout_ms,
            );

            video_data.p_instance = Some(video.assume_init());
            audio_data.p_instance = Some(audio.assume_init());

            response
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
        if let Some(mut x) = video_data.p_instance {
            unsafe {
                NDIlib_recv_free_video_v2(self.p_instance, &mut x);
            }
        }
    }

    pub fn free_audio_data(&self, audio_data: AudioData) {
        if let Some(mut x) = audio_data.p_instance {
            unsafe {
                NDIlib_recv_free_audio_v2(self.p_instance, &mut x);
            }
        }
    }
}

impl Drop for Recv {
    fn drop(&mut self) {
        unsafe { NDIlib_recv_destroy(self.p_instance) };
    }
}
