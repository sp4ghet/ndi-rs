use super::internal::bindings::*;
use super::*;
use core::panic;
use std::ffi::CString;
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

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum RecvColorFormat {
    BGRX_BGRA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_BGRX_BGRA,
    UYVY_BGRA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_UYVY_BGRA,
    RGBX_RGBA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_RGBX_RGBA,
    UYVY_RGBA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_UYVY_RGBA,
    Fastest = NDIlib_recv_color_format_e_NDIlib_recv_color_format_fastest,
    Best = NDIlib_recv_color_format_e_NDIlib_recv_color_format_best,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum RecvBandwidth {
    MetadataOnly = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_metadata_only,
    AudioOnly = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_audio_only,
    Lowest = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_lowest,
    Highest = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_highest,
}

#[derive(Debug, Clone)]
pub struct RecvBuilder {
    pub source_to_connect_to: Option<Source>,
    pub color_format: Option<RecvColorFormat>,
    pub bandwidth: Option<RecvBandwidth>,
    pub allow_video_fields: Option<bool>,
    pub ndi_recv_name: Option<String>,
}

impl RecvBuilder {
    pub fn new() -> Self {
        Self {
            source_to_connect_to: None,
            color_format: None,
            bandwidth: None,
            allow_video_fields: None,
            ndi_recv_name: None,
        }
    }

    pub fn source_to_connect_to(mut self, source: Source) -> Self {
        self.source_to_connect_to = Some(source);
        self
    }

    pub fn color_format(mut self, color_format: RecvColorFormat) -> Self {
        self.color_format = Some(color_format);
        self
    }

    pub fn bandwidth(mut self, bandwidth: RecvBandwidth) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    pub fn allow_video_fields(mut self, allow_video_fields: bool) -> Self {
        self.allow_video_fields = Some(allow_video_fields);
        self
    }

    pub fn ndi_recv_name(mut self, ndi_recv_name: String) -> Self {
        self.ndi_recv_name = Some(ndi_recv_name);
        self
    }

    pub fn build(self) -> Result<Recv, String> {
        // From default C++ constructor in Processing.NDI.Recv.h
        let mut settings: NDIlib_recv_create_v3_t = NDIlib_recv_create_v3_t {
            source_to_connect_to: Source::new().p_instance,
            color_format: RecvColorFormat::UYVY_BGRA as _,
            bandwidth: RecvBandwidth::Highest as _,
            allow_video_fields: true,
            p_ndi_recv_name: NULL as _,
        };

        if let Some(src) = self.source_to_connect_to {
            settings.source_to_connect_to = src.p_instance;
        }
        if let Some(color_format) = self.color_format {
            settings.color_format = color_format as _;
        }
        if let Some(bandwidth) = self.bandwidth {
            settings.bandwidth = bandwidth as _;
        }
        if let Some(allow_video_fields) = self.allow_video_fields {
            settings.allow_video_fields = allow_video_fields;
        }
        if let Some(ndi_recv_name) = self.ndi_recv_name {
            let cstr = CString::new(ndi_recv_name).map_err(|x| x.to_string())?;
            settings.p_ndi_recv_name = cstr.as_ptr();
        }

        Recv::with_settings(settings)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecvQueueSize {
    pub video_frames: u32,
    pub audio_frames: u32,
    pub metadata_frames: u32,
}

impl RecvQueueSize {
    pub fn new() -> Self {
        Self {
            video_frames: 0,
            audio_frames: 0,
            metadata_frames: 0,
        }
    }

    fn from_binding(queue: NDIlib_recv_queue_t) -> Self {
        Self {
            video_frames: queue.video_frames as _,
            audio_frames: queue.audio_frames as _,
            metadata_frames: queue.metadata_frames as _,
        }
    }
}

pub struct Recv {
    p_instance: NDIlib_recv_instance_t,
}

impl Recv {
    fn with_settings(settings: NDIlib_recv_create_v3_t) -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_recv_create_v3(&settings) };

        if p_instance.is_null() {
            return Err("Failed to create NDI Recv instance".to_string());
        }

        Ok(Self { p_instance })
    }

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
        video_data: &mut Option<VideoData>,
        audio_data: &mut Option<AudioData>,
        timeout_ms: u32,
    ) -> FrameType {
        let response = unsafe {
            let mut video = if let Some(x) = video_data {
                mem::MaybeUninit::new(x.p_instance)
            } else {
                mem::MaybeUninit::uninit()
            };
            let mut audio = if let Some(x) = audio_data {
                mem::MaybeUninit::new(x.p_instance)
            } else {
                mem::MaybeUninit::uninit()
            };
            let response = NDIlib_recv_capture_v3(
                self.p_instance,
                video.as_mut_ptr(),
                audio.as_mut_ptr(),
                NULL as _,
                timeout_ms,
            );

            *video_data = Some(VideoData::from_binding(video.assume_init()));
            *audio_data = Some(AudioData::from_binding(audio.assume_init()));

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

    // pub fn get_error(&self) -> String {
    //     let res = unsafe {
    //         let char_ptr = NDIlib_recv_recording_get_error(self.p_instance);
    //         if char_ptr.is_null() {
    //             return String::new();
    //         }
    //         CStr::from_ptr(char_ptr).to_string_lossy().to_string()
    //     };
    //     res
    // }

    pub fn set_tally(&mut self, tally: Tally) {
        unsafe {
            NDIlib_recv_set_tally(self.p_instance, &tally);
        }
    }

    pub fn get_queue(&self) -> RecvQueueSize {
        let mut p_total: mem::MaybeUninit<NDIlib_recv_queue_t> = mem::MaybeUninit::uninit();
        unsafe {
            NDIlib_recv_get_queue(self.p_instance, p_total.as_mut_ptr());
            let queue = RecvQueueSize::from_binding(p_total.assume_init());

            queue
        }
    }

    pub fn free_metadata(&self, mut metadata: MetaData) {
        unsafe {
            NDIlib_recv_free_metadata(self.p_instance, &mut metadata.p_instance);
        }
    }

    pub fn free_video_data(&self, mut video_data: VideoData) {
        unsafe {
            NDIlib_recv_free_video_v2(self.p_instance, &mut video_data.p_instance);
        }
    }

    pub fn free_audio_data(&self, mut audio_data: AudioData) {
        unsafe {
            NDIlib_recv_free_audio_v3(self.p_instance, &mut audio_data.p_instance);
        }
    }
}

impl Drop for Recv {
    fn drop(&mut self) {
        unsafe { NDIlib_recv_destroy(self.p_instance) };
    }
}
