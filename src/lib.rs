//! NewTek NDI Bindings for rust

use core::panic;
use internal::bindings::*;
use std::{convert::TryFrom, ffi::CStr, fmt::Debug};

pub mod find;
pub mod internal;
pub mod recv;
pub mod send;

pub use find::*;
pub use recv::*;
pub use send::*;

const NULL: usize = 0;

/// A description of the type of of frame received.
///
/// This is usually returned by [`Recv::capture`]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    None = NDIlib_frame_type_e_NDIlib_frame_type_none,
    Video = NDIlib_frame_type_e_NDIlib_frame_type_video,
    Audio = NDIlib_frame_type_e_NDIlib_frame_type_audio,
    StatusChange = NDIlib_frame_type_e_NDIlib_frame_type_status_change,
    ErrorFrame = NDIlib_frame_type_e_NDIlib_frame_type_error,
    Metadata = NDIlib_frame_type_e_NDIlib_frame_type_metadata,
}

impl TryFrom<i32> for FrameType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value {
            NDIlib_frame_type_e_NDIlib_frame_type_audio => Ok(FrameType::Audio),
            NDIlib_frame_type_e_NDIlib_frame_type_video => Ok(FrameType::Video),
            NDIlib_frame_type_e_NDIlib_frame_type_none => Ok(FrameType::None),
            NDIlib_frame_type_e_NDIlib_frame_type_status_change => Ok(FrameType::StatusChange),
            NDIlib_frame_type_e_NDIlib_frame_type_error => Ok(FrameType::ErrorFrame),
            NDIlib_frame_type_e_NDIlib_frame_type_metadata => Ok(FrameType::Metadata),
            x => Err(format!("Unknown NDI frame type encountered: {}", x)),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum FrameFormatType {
    Progressive = NDIlib_frame_format_type_e_NDIlib_frame_format_type_progressive,
    Interleaved = NDIlib_frame_format_type_e_NDIlib_frame_format_type_interleaved,
    Field0 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_0,
    Field1 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_1,
}

impl TryFrom<i32> for FrameFormatType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value {
            NDIlib_frame_format_type_e_NDIlib_frame_format_type_progressive => {
                Ok(FrameFormatType::Progressive)
            }

            NDIlib_frame_format_type_e_NDIlib_frame_format_type_interleaved => {
                Ok(FrameFormatType::Interleaved)
            }

            NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_0 => {
                Ok(FrameFormatType::Field0)
            }
            NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_1 => {
                Ok(FrameFormatType::Field1)
            }
            x => Err(format!("Unknown NDI video data frame format type: {}", x)),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
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

impl TryFrom<i32> for FourCCVideoType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value {
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVY => Ok(FourCCVideoType::UYVY),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVA => Ok(FourCCVideoType::UYVA),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_P216 => Ok(FourCCVideoType::P216),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_PA16 => Ok(FourCCVideoType::PA16),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_YV12 => Ok(FourCCVideoType::YV12),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_I420 => Ok(FourCCVideoType::I420),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_NV12 => Ok(FourCCVideoType::NV12),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRA => Ok(FourCCVideoType::BGRA),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBA => Ok(FourCCVideoType::RGBA),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRX => Ok(FourCCVideoType::BGRX),
            NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBX => Ok(FourCCVideoType::RGBX),
            x => Err(format!("Unknown FourCC video type encountered: {}", x)),
        }
    }
}

#[repr(i32)]
pub enum FourCCAudioType {
    FLTP = NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP,
}

#[derive(Clone)]
pub struct Source {
    p_instance: NDIlib_source_t,
}

impl Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ndi::Source")
            .field("name", &self.get_name())
            .finish()
    }
}

impl Source {
    fn from_binding(source: NDIlib_source_t) -> Self {
        Self { p_instance: source }
    }

    pub fn new() -> Self {
        // From the default c++ constructor in Processing.NDI.structs.h
        let p_instance = NDIlib_source_t {
            p_ndi_name: NULL as _,
            __bindgen_anon_1: NDIlib_source_t__bindgen_ty_1 {
                p_ip_address: NULL as _,
            },
        };
        Self { p_instance }
    }

    pub fn get_name(&self) -> Result<String, String> {
        let name_char_ptr: *mut std::os::raw::c_char = self.p_instance.p_ndi_name as _;
        if name_char_ptr.is_null() {
            return Ok(String::new());
        }
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

pub type Tally = NDIlib_tally_t;

impl Tally {
    pub fn new() -> Self {
        Self {
            on_program: false,
            on_preview: false,
        }
    }
}

impl Default for Tally {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VideoData {
    p_instance: NDIlib_video_frame_v2_t,
}

impl VideoData {
    pub fn from_binding(p_instance: NDIlib_video_frame_v2_t) -> Self {
        Self { p_instance }
    }

    pub fn xres(&self) -> u32 {
        self.p_instance.xres as _
    }

    pub fn yres(&self) -> u32 {
        self.p_instance.yres as _
    }

    pub fn four_cc(&self) -> FourCCVideoType {
        FourCCVideoType::try_from(self.p_instance.FourCC).unwrap()
    }

    pub fn frame_rate_n(&self) -> u32 {
        self.p_instance.frame_rate_N as _
    }

    pub fn frame_rate_d(&self) -> u32 {
        self.p_instance.frame_rate_D as _
    }

    pub fn picture_aspect_ratio(&self) -> f32 {
        self.p_instance.picture_aspect_ratio
    }
    pub fn frame_format_type(&self) -> FrameFormatType {
        FrameFormatType::try_from(self.p_instance.FourCC).unwrap()
    }

    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }
    pub fn p_data(&self) -> *mut u8 {
        self.p_instance.p_data
    }

    pub fn metadata(&self) -> String {
        let metadata_char_ptr = self.p_instance.p_metadata;
        if metadata_char_ptr.is_null() {
            return String::new();
        }
        let metadata = unsafe { CStr::from_ptr(metadata_char_ptr) }
            .to_owned()
            .to_string_lossy()
            .to_string();
        metadata
    }

    pub fn timestamp(&self) -> i64 {
        self.p_instance.timestamp
    }
}

pub struct AudioData {
    p_instance: NDIlib_audio_frame_v3_t,
}

impl AudioData {
    pub fn from_binding(p_instance: NDIlib_audio_frame_v3_t) -> Self {
        Self { p_instance }
    }

    pub fn sample_rate(&self) -> u32 {
        self.p_instance.sample_rate as _
    }

    pub fn no_channels(&self) -> u32 {
        self.p_instance.no_channels as _
    }

    pub fn no_samples(&self) -> u32 {
        self.p_instance.no_samples as _
    }

    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }

    pub fn timestamp(&self) -> i64 {
        self.p_instance.timestamp
    }

    pub fn p_data(&self) -> *mut u8 {
        self.p_instance.p_data
    }

    pub fn four_cc(&self) -> FourCCAudioType {
        #[allow(non_upper_case_globals)]
        match self.p_instance.FourCC {
            NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP => FourCCAudioType::FLTP,
            x => panic!("Unknown NDI FourCC Audio type encountered: {}", x),
        }
    }

    pub fn channel_stride_in_bytes(&self) -> u32 {
        match self.four_cc() {
            FourCCAudioType::FLTP => unsafe {
                self.p_instance.__bindgen_anon_1.channel_stride_in_bytes as _
            },
        }
    }

    pub fn metadata(&self) -> String {
        let metadata_char_ptr = self.p_instance.p_metadata;
        if metadata_char_ptr.is_null() {
            return String::new();
        }
        let metadata = unsafe { CStr::from_ptr(metadata_char_ptr) }
            .to_owned()
            .to_string_lossy()
            .to_string();
        metadata
    }
}

#[derive(Debug)]
pub struct MetaData {
    p_instance: NDIlib_metadata_frame_t,
}

impl MetaData {
    pub fn from_binding(p_instance: NDIlib_metadata_frame_t) -> Self {
        Self { p_instance }
    }

    pub fn length(&self) -> u32 {
        self.p_instance.length as _
    }
    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }
    pub fn p_data(&self) -> String {
        // according to the docs, metadata should be valid UTF-8 XML
        // not sure how much it's actually followed in practice
        let char_ptr = self.p_instance.p_data;
        let data = unsafe { CStr::from_ptr(char_ptr).to_string_lossy().to_string() };
        data
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
