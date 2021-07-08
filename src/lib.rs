use internal::bindings::*;
use std::ffi::CStr;

pub mod find;
pub mod internal;
pub mod recv;

pub use find::*;
pub use recv::*;

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
    fn from_binding(source: NDIlib_source_t) -> Self {
        Self { p_instance: source }
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
    p_instance: Option<NDIlib_video_frame_v2_t>,
}

impl VideoData {
    pub fn new() -> Self {
        let p_instance = None;
        Self { p_instance }
    }
}

pub struct AudioData {
    p_instance: Option<NDIlib_audio_frame_v2_t>,
}

impl AudioData {
    pub fn new() -> Self {
        let p_instance = None;

        Self { p_instance }
    }
}

pub fn initialize() -> Result<(), String> {
    if !unsafe { NDIlib_initialize() } {
        return Err("Failed to initialize NDIlib".to_string());
    };

    Ok(())
}

pub fn cleanup() {
    unsafe { NDIlib_destroy() };
}
