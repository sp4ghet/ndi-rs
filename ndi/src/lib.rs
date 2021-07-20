#![warn(missing_docs)]
//! NewTek NDI®[^tm] Bindings for rust
//!
//! [^tm]: NDI® is a registered trademark of NewTek, Inc.
//! http://ndi.tv/
//!

use core::panic;
use internal::bindings::*;
use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    fmt::{Debug, Display},
    sync::Arc,
};

/// The error type used in this crate
pub mod error;
/// The [`Find`] struct and related constructs for finding NDI sources
pub mod find;
#[doc(hidden)]
pub mod internal;
/// The [`Recv`] struct and related constructs for receiving NDI
pub mod recv;
/// The [`Send`] struct and related constructs for sending NDI
pub mod send;

#[doc(hidden)]
pub use error::*;
#[doc(hidden)]
pub use find::*;
#[doc(hidden)]
pub use recv::*;
#[doc(hidden)]
pub use send::*;

const NULL: usize = 0;

/// A description of the type of of frame received.
///
/// This is usually returned by [`Recv::capture_all()`]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// nothing changed, usually due to timeout
    None = NDIlib_frame_type_e_NDIlib_frame_type_none as _,
    /// Received a video frame
    Video = NDIlib_frame_type_e_NDIlib_frame_type_video as _,
    /// Received an audio frame
    Audio = NDIlib_frame_type_e_NDIlib_frame_type_audio as _,
    /// Received a metadata frame
    Metadata = NDIlib_frame_type_e_NDIlib_frame_type_metadata as _,
    /// This indicates that the settings on this input have changed.
    /// For instance, this value will be returned from [`recv::Recv::capture_all()`].
    /// when the device is known to have new settings, for instance the web URL has changed or the device
    /// is now known to be a PTZ camera.
    StatusChange = NDIlib_frame_type_e_NDIlib_frame_type_status_change as _,
    /// error occured (disconnected)
    ErrorFrame = NDIlib_frame_type_e_NDIlib_frame_type_error as _,
}

impl TryFrom<NDIlib_frame_type_e> for FrameType {
    type Error = NDIError;

    fn try_from(value: NDIlib_frame_type_e) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value {
            NDIlib_frame_type_e_NDIlib_frame_type_audio => Ok(FrameType::Audio),
            NDIlib_frame_type_e_NDIlib_frame_type_video => Ok(FrameType::Video),
            NDIlib_frame_type_e_NDIlib_frame_type_none => Ok(FrameType::None),
            NDIlib_frame_type_e_NDIlib_frame_type_status_change => Ok(FrameType::StatusChange),
            NDIlib_frame_type_e_NDIlib_frame_type_error => Ok(FrameType::ErrorFrame),
            NDIlib_frame_type_e_NDIlib_frame_type_metadata => Ok(FrameType::Metadata),
            x => Err(NDIError::InvalidEnum(x as _, "FrameType")),
        }
    }
}

/// A description of the frome format of a frame
///
/// This is usually part of a [`VideoData`] frame.
///
/// To make everything as easy to use as possible, the SDK always assumes that fields are ‘top field first’.
/// This is, in fact, the case for every modern format, but does create a problem
/// for two specific older video formats as discussed below:
///
/// #### NTSC 486 LINES
/// The best way to handle this format is simply to offset the image vertically by one line (`p_uyvy_data + uyvy_stride_in_bytes`)
/// and reduce the vertical resolution to 480 lines. This can all be done without modification
/// of the data being passed in at all; simply change the data and resolution pointers.
///
/// #### DV NTSC
/// This format is a relatively rare these days, although still used from time to time. There is no entirely trivial way to
/// handle this other than to move the image down one line and add a black line at the bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormatType {
    /// This is a progressive video frame
    Progressive = NDIlib_frame_format_type_e_NDIlib_frame_format_type_progressive as _,
    /// This is a frame of video that is comprised of two fields.
    ///
    ///  The upper field comes first, and the lower comes second (see [`FrameFormatType`])
    Interleaved = NDIlib_frame_format_type_e_NDIlib_frame_format_type_interleaved as _,
    /// This is an individual field 0 from a fielded video frame.
    ///
    /// This is the first temporal, upper field. (see [`FrameFormatType`])
    Field0 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_0 as _,
    /// This is an individual field 1 from a fielded video frame.
    ///
    /// This is the second temporal, lower field (see [`FrameFormatType`])
    Field1 = NDIlib_frame_format_type_e_NDIlib_frame_format_type_field_1 as _,
}

impl TryFrom<NDIlib_frame_format_type_e> for FrameFormatType {
    type Error = NDIError;

    fn try_from(value: NDIlib_frame_format_type_e) -> Result<Self, Self::Error> {
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
            x => Err(NDIError::InvalidEnum(x as _, "FrameFormatType")),
        }
    }
}

/// The [FourCC](https://www.fourcc.org/) type of a [`VideoData`] frame
///
/// When running in a YUV color space, the following standards are applied:
///
/// | Resolution | Standard |
/// | -------- | ------- |
/// | SD Resolutions | BT.601 |
/// | HD resolutions >(720,576) | Rec.709 |
/// | UHD resolutions > (1920,1080) | Rec.2020 |
/// | Alpha | Full range for data type (2^8 for 8-bit, 2^16 for 16-bit) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FourCCVideoType {
    /// A buffer in the “UYVY” FourCC and represents a 4:2:2 image in YUV color space.
    ///
    /// There is a Y sample at every pixel, and U and V sampled at
    /// every second pixel horizontally on each line. A macro-pixel contains 2
    /// pixels in 1 DWORD. The ordering of these pixels is U0, Y0, V0, Y1.
    UYVY = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVY as _,

    /// A buffer that represents a 4:2:2:4 image in YUV color space.
    ///
    /// There is a Y sample at every pixels with U,V sampled at every second pixel
    /// horizontally. There are two planes in memory, the first being the UYVY
    /// color plane, and the second the alpha plane that immediately follows the first.
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint8_t *p_uyvy = (uint8_t*)p_data;
    /// uint8_t *p_alpha = p_uyvy + stride*yres;
    /// ```
    UYVA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_UYVA as _,
    /// A 4:2:2 buffer in semi-planar format with full 16bpp color precision.
    ///
    /// This is formed from two buffers in memory, the first is a 16bpp
    /// luminance buffer and the second is a buffer of U,V pairs in memory. This
    /// can be considered as a 16bpp version of NV12.
    ///
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint16_t *p_y = (uint16_t*)p_data;
    /// uint16_t *p_uv = (uint16_t*)(p_data + stride*yres);
    /// ```
    /// As a matter of illustration, a completely packed image would have stride as `xres*sizeof(uint16_t)`.
    P216 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_P216 as _,
    /// A 4:2:2:4 buffer in semi-planar format with full 16bpp color and alpha precision.
    ///
    /// This is formed from three buffers in memory. The first is
    /// a 16bpp luminance buffer, and the second is a buffer of U,V pairs in
    /// memory. A single plane alpha channel at 16bpp follows the U,V pairs.
    ///
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint16_t *p_y = (uint16_t*)p_data;
    /// uint16_t *p_uv = p_y + stride*yres;
    /// uint16_t *p_alpha = p_uv + stride*yres;
    /// ```
    /// To illustrate, a completely packed image would have stride as `xres*sizeof(uint16_t)`.
    PA16 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_PA16 as _,
    /// A planar 4:2:0 in Y, U, V planes in memory.
    ///
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint8_t *p_y = (uint8_t*)p_data;
    /// uint8_t *p_u = p_y + stride*yres;
    /// uint8_t *p_v = p_u + (stride/2)*(yres/2);
    /// As a matter of illustration, a completely packed image would have stride as `xres*sizeof(uint8_t)`.
    YV12 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_YV12 as _,
    /// A planar 4:2:0 in Y, U, V planes in memory with the U, V planes reversed from the YV12 format.
    ///
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint8_t *p_y = (uint8_t*)p_data;
    /// uint8_t *p_v = p_y + stride*yres;
    /// uint8_t *p_u = p_v + (stride/2)*(yres/2);
    /// ```
    /// To illustrate, a completely packed image would have stride as `xres*sizeof(uint8_t)`.
    I420 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_I420 as _,
    /// A semi planar 4:2:0 in Y, UV planes in memory.
    ///
    /// The luminance plane is at the lowest memory address with the UV pairs immediately following them.
    ///
    /// For instance, if you have an image with p_data and stride, then the planes are located as follows:
    /// ```c++
    /// uint8_t *p_y = (uint8_t*)p_data;
    /// uint8_t *p_uv = p_y + stride*yres;
    /// ```
    /// To illustrate, a completely packed image would have stride as `xres*sizeof(uint8_t)`.
    NV12 = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_NV12 as _,
    /// A 4:4:4:4, 8-bit image of red, green, blue and alpha components
    ///
    /// in memory order blue, green, red, alpha. This data is not pre-multiplied.
    BGRA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRA as _,
    /// A 4:4:4, 8-bit image of red, green, blue components
    ///  in memory order blue, green, red, 255. This data is not pre-multiplied.
    ///
    /// This is identical to BGRA, but is provided as a hint that all alpha channel
    /// values are 255, meaning that alpha compositing may be avoided. The lack
    /// of an alpha channel is used by the SDK to improve performance when possible.
    BGRX = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_BGRX as _,
    /// A 4:4:4:4, 8-bit image of red, green, blue and alpha components
    ///
    /// in memory order red, green, blue, alpha. This data is not pre-multiplied.
    RGBA = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBA as _,
    /// A 4:4:4, 8-bit image of red, green, blue components
    ///
    /// in memory order red, green, blue, 255. This data is not pre-multiplied.
    ///This is identical to RGBA, but is provided as a hint that all alpha channel
    ///values are 255, meaning that alpha compositing may be avoided. The lack
    ///of an alpha channel is used by the SDK to improve performance when possible.
    RGBX = NDIlib_FourCC_video_type_e_NDIlib_FourCC_type_RGBX as _,
}

impl TryFrom<NDIlib_FourCC_video_type_e> for FourCCVideoType {
    type Error = NDIError;

    fn try_from(value: NDIlib_FourCC_video_type_e) -> Result<Self, Self::Error> {
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
            x => Err(NDIError::InvalidEnum(x as _, "FourCCVideoType")),
        }
    }
}

/// The [FourCC](https://www.fourcc.org/) type of a [`AudioData`] frame
#[derive(Debug, Clone, Copy)]
pub enum FourCCAudioType {
    /// This format stands for floating-point audio.
    FLTP = NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP as _,
}

impl TryFrom<NDIlib_FourCC_audio_type_e> for FourCCAudioType {
    type Error = NDIError;

    fn try_from(value: NDIlib_FourCC_audio_type_e) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value {
            NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP => Ok(FourCCAudioType::FLTP),
            x => Err(NDIError::InvalidEnum(x as _, "FourCCAudioType")),
        }
    }
}

/// A descriptor of a NDI source available on the network.
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

    fn new() -> Self {
        // From the default c++ constructor in Processing.NDI.structs.h
        let p_instance = NDIlib_source_t {
            p_ndi_name: NULL as _,
            __bindgen_anon_1: NDIlib_source_t__bindgen_ty_1 {
                p_ip_address: NULL as _,
            },
        };
        Self { p_instance }
    }

    /// A UTF8 string that provides a user readable name for this source.
    ///
    /// This can be used for serialization, etc... and comprises the machine
    /// name and the source name on that machine. In the form
    ///     MACHINE_NAME (NDI_SOURCE_NAME)
    /// If the parameter was passed either as NULL, or an EMPTY string then
    /// the specific IP address and port number from below is used.
    pub fn get_name(&self) -> Result<String, std::str::Utf8Error> {
        let name_char_ptr: *mut std::os::raw::c_char = self.p_instance.p_ndi_name as _;
        if name_char_ptr.is_null() {
            return Ok(String::new());
        }
        let name = unsafe {
            CStr::from_ptr(name_char_ptr)
                .to_owned()
                .to_str()?
                .to_string()
        };
        Ok(name)
    }
}

unsafe impl core::marker::Send for Source {}
unsafe impl core::marker::Sync for Source {}

/// Tally information
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tally {
    /// Is this currently on program output
    pub on_program: bool,
    /// Is this currently on preview output
    pub on_preview: bool,
}

impl Tally {
    /// Create a new [`Tally`] instance.
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

impl From<NDIlib_tally_t> for Tally {
    fn from(x: NDIlib_tally_t) -> Self {
        Self {
            on_preview: x.on_preview,
            on_program: x.on_program,
        }
    }
}

impl Into<NDIlib_tally_t> for Tally {
    fn into(self) -> NDIlib_tally_t {
        NDIlib_tally_t {
            on_preview: self.on_preview,
            on_program: self.on_program,
        }
    }
}

enum VideoParent {
    Recv(Arc<NDIlib_recv_instance_t>),
    #[allow(unused)]
    Owned,
}

/// Describes a video frame
pub struct VideoData {
    p_instance: NDIlib_video_frame_v2_t,
    parent: VideoParent,
}

unsafe impl core::marker::Send for VideoData {}
unsafe impl core::marker::Sync for VideoData {}

impl Debug for VideoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoData")
            .field("xres", &self.xres())
            .field("yres", &self.yres())
            .field("line_stride_in_bytes", &self.line_stride_in_bytes())
            .field("data_size", &self.data_size_in_bytes())
            .field("fourcc", &self.four_cc())
            .field("frame_format_type", &self.frame_format_type())
            .field(
                "frame_rate",
                &format!(
                    "{}/{} = {}",
                    self.frame_rate_n(),
                    self.frame_rate_d(),
                    self.frame_rate_n() as f32 / self.frame_rate_d() as f32
                ),
            )
            .field("timestamp", &self.timestamp())
            .field("timecode", &self.timecode())
            .field("metadata", &self.metadata())
            .finish()
    }
}

impl VideoData {
    fn from_binding_recv(
        recv: Arc<NDIlib_recv_instance_t>,
        p_instance: NDIlib_video_frame_v2_t,
    ) -> Self {
        Self {
            p_instance,
            parent: VideoParent::Recv(recv),
        }
    }

    /// Create an empty video frame
    pub fn new() -> Self {
        Self {
            p_instance: NDIlib_video_frame_v2_t {
                xres: 0,
                yres: 0,
                FourCC: FourCCVideoType::UYVY as _,
                frame_rate_N: 60,
                frame_rate_D: 0,
                picture_aspect_ratio: 0f32,
                frame_format_type: FrameFormatType::Progressive as _,
                timecode: 0,
                p_data: NULL as _,
                __bindgen_anon_1: NDIlib_video_frame_v2_t__bindgen_ty_1 {
                    line_stride_in_bytes: 0,
                },
                p_metadata: NULL as _,
                timestamp: 0,
            },
            parent: VideoParent::Owned,
        }
    }

    /// The width of the frame expressed in pixels.
    ///
    /// Note that, because data is internally all considered
    /// in 4:2:2 formats, image width values
    /// should be divisible by two.
    pub fn xres(&self) -> u32 {
        self.p_instance.xres as _
    }

    /// The height of the frame expressed in pixels.
    pub fn yres(&self) -> u32 {
        self.p_instance.yres as _
    }

    /// The FourCC pixel format for this buffer.
    ///
    /// See [`FourCCVideoType`] for details on possible values
    pub fn four_cc(&self) -> FourCCVideoType {
        FourCCVideoType::try_from(self.p_instance.FourCC).unwrap()
    }

    /// The numerator of the framerate of the current frame.
    ///
    /// The framerate is specified as a
    /// numerator and denominator, such that the following is valid:
    /// ```
    /// # let frame_rate_N = 0 as u32;
    /// # let frame_rate_D = 1 as u32;
    /// let frame_rate = (frame_rate_N as f32) / (frame_rate_D as f32);
    /// ```

    pub fn frame_rate_n(&self) -> u32 {
        self.p_instance.frame_rate_N as _
    }

    /// The denominator of the framerate of the current frame.
    ///
    /// The framerate is specified as a
    /// numerator and denominator, such that the following is valid:
    /// ```
    /// # let frame_rate_N = 0 as u32;
    /// # let frame_rate_D = 1 as u32;
    /// let frame_rate = (frame_rate_N as f32) / (frame_rate_D as f32);
    /// ```
    pub fn frame_rate_d(&self) -> u32 {
        self.p_instance.frame_rate_D as _
    }

    /// The SDK defines picture aspect ratio (as opposed to pixel aspect ratios).
    ///
    /// When the aspect ratio is 0.0 it is interpreted as xres/yres,
    ///  or square pixel; for most modern video types this is a default that can be used.
    pub fn picture_aspect_ratio(&self) -> f32 {
        self.p_instance.picture_aspect_ratio
    }

    /// The frame format type of a video
    pub fn frame_format_type(&self) -> FrameFormatType {
        FrameFormatType::try_from(self.p_instance.frame_format_type).unwrap()
    }

    /// The timecode of this frame in 100 ns intervals.
    ///
    /// This is generally not used internally by the SDK
    ///  but is passed through to applications, which may interpret it as they wish.
    /// See [`Send`] for details
    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }

    /// The video data itself laid out linearly in memory
    ///
    /// The memory is laid out in the FourCC format returned by `four_cc()`.
    /// The number of bytes defined between lines is specified in `line_stride_in_bytes()`
    pub fn p_data(&self) -> *mut u8 {
        self.p_instance.p_data
    }

    /// This is the inter-line stride of the video data, in bytes.
    pub fn line_stride_in_bytes(&self) -> Option<u32> {
        // TODO: detect whether it is a compressed type

        // If the FourCC is not a compressed type, then this will be the
        // inter-line stride of the video data in bytes. If the stride is 0,
        // then it will default to sizeof(one pixel)*xres.
        unsafe { Some(self.p_instance.__bindgen_anon_1.line_stride_in_bytes as _) }
    }

    /// The size of the p_data buffer in bytes.
    pub fn data_size_in_bytes(&self) -> Option<u32> {
        // If the FourCC is a compressed type, then this will be the size of the
        // p_data buffer in bytes.
        unsafe { Some(self.p_instance.__bindgen_anon_1.data_size_in_bytes as _) }
    }

    /// A per frame metadata stream that should be XML
    ///
    /// It is sent and received with the frame.
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

    /// A per-frame timestamp filled in by the NDI SDK using a high precision clock.
    ///
    /// This is only valid when receiving a frame.
    /// It represents the time (in 100 ns intervals measured in UTC time,
    /// since the Unix Time Epoch 1/1/1970 00:00 of
    /// the exact moment that the frame was submitted by the sending side
    /// If this value is None then this value is not available.
    pub fn timestamp(&self) -> Option<i64> {
        let timestamp = self.p_instance.timestamp;
        if timestamp == NDIlib_recv_timestamp_undefined {
            None
        } else {
            Some(timestamp)
        }
    }
}

impl Drop for VideoData {
    fn drop(&mut self) {
        match &self.parent {
            VideoParent::Recv(recv) => unsafe {
                NDIlib_recv_free_video_v2(**recv, &mut self.p_instance);
            },
            VideoParent::Owned => {}
        }
    }
}

enum AudioParent {
    Recv(Arc<NDIlib_recv_instance_t>),
    #[allow(unused)]
    Owned,
}

/// An audio frame
pub struct AudioData {
    p_instance: NDIlib_audio_frame_v3_t,
    parent: AudioParent,
}

unsafe impl core::marker::Send for AudioData {}
unsafe impl core::marker::Sync for AudioData {}

impl Debug for AudioData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioData")
            .field("sample_rate", &self.sample_rate())
            .field("no_samples", &self.no_samples())
            .field("no_channels", &self.no_channels())
            .field("timestamp", &self.timestamp())
            .field("timecode", &self.timecode())
            .field("fourcc", &self.four_cc())
            .field("metadata", &self.metadata())
            .finish()
    }
}

impl AudioData {
    fn from_binding_recv(
        recv: Arc<NDIlib_recv_instance_t>,
        p_instance: NDIlib_audio_frame_v3_t,
    ) -> Self {
        Self {
            p_instance,
            parent: AudioParent::Recv(recv),
        }
    }

    /// Create new instance of AudioData
    pub fn new() -> Self {
        Self {
            p_instance: NDIlib_audio_frame_v3_t {
                sample_rate: 0,
                no_channels: 0,
                no_samples: 0,
                timecode: 0,
                FourCC: FourCCAudioType::FLTP as _,
                p_data: NULL as _,
                __bindgen_anon_1: NDIlib_audio_frame_v3_t__bindgen_ty_1 {
                    channel_stride_in_bytes: 0,
                },
                p_metadata: "".as_ptr() as _,
                timestamp: 0,
            },
            parent: AudioParent::Owned,
        }
    }

    /// The sample-rate of this buffer
    pub fn sample_rate(&self) -> u32 {
        self.p_instance.sample_rate as _
    }

    /// The number of audio channels
    pub fn no_channels(&self) -> u32 {
        self.p_instance.no_channels as _
    }

    /// The number of audio samples per channel
    pub fn no_samples(&self) -> u32 {
        self.p_instance.no_samples as _
    }

    /// The timecode of this frame in 100ns intervals
    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }

    /// A per-frame timestamp filled in by the NDI SDK using a high precision clock.
    ///
    /// This is only valid when receiving a frame.
    /// It represents the time (in 100 ns intervals measured in UTC time,
    /// since the Unix Time Epoch 1/1/1970 00:00 of
    /// the exact moment that the frame was submitted by the sending side
    /// If this value is None then this value is not available.
    pub fn timestamp(&self) -> Option<i64> {
        let timestamp = self.p_instance.timestamp;
        if timestamp == NDIlib_recv_timestamp_undefined {
            None
        } else {
            Some(timestamp)
        }
    }

    /// A pointer to the audio data
    ///
    /// If FourCC is NDIlib_FourCC_type_FLTP, then this is the floating-point
    /// audio data in planar format, with each audio channel stored
    /// together with a stride between channels specified by
    /// channel_stride_in_bytes.
    pub fn p_data(&self) -> *mut u8 {
        self.p_instance.p_data
    }

    /// What FourCC type is for this frame
    ///
    /// There is currently one supported format: FLTP.
    pub fn four_cc(&self) -> FourCCAudioType {
        #[allow(non_upper_case_globals)]
        match self.p_instance.FourCC {
            NDIlib_FourCC_audio_type_e_NDIlib_FourCC_type_FLTP => FourCCAudioType::FLTP,
            x => panic!("Unknown NDI FourCC Audio type encountered: {}", x),
        }
    }

    /// The stride in bytes for a single channel.
    ///
    /// This is the number of bytes that are used to step from one audio
    /// channel to another.
    ///
    pub fn channel_stride_in_bytes(&self) -> u32 {
        match self.four_cc() {
            FourCCAudioType::FLTP => unsafe {
                self.p_instance.__bindgen_anon_1.channel_stride_in_bytes as _
            },
        }
    }

    /// This is a per frame metadata stream in XML
    ///
    /// It is sent and received with the frame.
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

impl Drop for AudioData {
    fn drop(&mut self) {
        match &self.parent {
            AudioParent::Recv(recv) => unsafe {
                NDIlib_recv_free_audio_v3(**recv, &self.p_instance);
            },
            AudioParent::Owned => {}
        }
    }
}

enum MetaDataParent {
    Recv(Arc<NDIlib_recv_instance_t>),
    Send(Arc<NDIlib_send_instance_t>),
    Owned,
}

/// The data description for metadata
pub struct MetaData {
    p_instance: NDIlib_metadata_frame_t,
    parent: MetaDataParent,
}

unsafe impl core::marker::Send for MetaData {}
unsafe impl core::marker::Sync for MetaData {}

impl Debug for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaData")
            .field("length", &self.length())
            .field("data", &self.data())
            .field("timecode", &self.timecode())
            .finish()
    }
}

impl MetaData {
    fn from_binding_recv(
        recv: Arc<NDIlib_recv_instance_t>,
        p_instance: NDIlib_metadata_frame_t,
    ) -> Self {
        Self {
            p_instance,
            parent: MetaDataParent::Recv(recv),
        }
    }

    fn from_binding_send(
        send: Arc<NDIlib_send_instance_t>,
        p_instance: NDIlib_metadata_frame_t,
    ) -> Self {
        Self {
            p_instance,
            parent: MetaDataParent::Send(send),
        }
    }

    /// Create new metadata struct
    pub fn new(length: u32, timecode: i64, data: String) -> Self {
        let p_data = CString::new(data).unwrap().into_raw();
        let p_instance = NDIlib_metadata_frame_t {
            length: length as _,
            timecode,
            p_data,
        };
        Self {
            p_instance,
            parent: MetaDataParent::Owned,
        }
    }

    /// The length of the string in UTF8 characters. This includes the NULL terminating character.
    /// If this is 0, then the length is assumed to be the length of a NULL terminated string.
    pub fn length(&self) -> u32 {
        self.p_instance.length as _
    }

    /// The timecode of this frame in 100ns intervals
    pub fn timecode(&self) -> i64 {
        self.p_instance.timecode
    }

    /// The metadata as a UTF8 XML string. This is a NULL terminated string.
    pub fn data(&self) -> String {
        //! according to the docs, metadata should be valid UTF-8 XML
        //! not sure how much it's actually followed in practice
        let char_ptr = self.p_instance.p_data;
        let data = unsafe { CStr::from_ptr(char_ptr).to_string_lossy().to_string() };
        data
    }
}

impl Drop for MetaData {
    fn drop(&mut self) {
        match &self.parent {
            MetaDataParent::Recv(recv) => unsafe {
                NDIlib_recv_free_metadata(**recv, &mut self.p_instance);
            },
            MetaDataParent::Send(send) => unsafe {
                NDIlib_send_free_metadata(**send, &mut self.p_instance);
            },
            MetaDataParent::Owned => {}
        }
    }
}

/// Start the library
///
/// This is not actually required, but will start the libraries which might get
/// you slightly better performance in some cases.In general it is more "correct" to
/// call this although it is not required. There is no way to call this that would have
/// an adverse impact on anything.
/// This will return Err if the CPU is not sufficiently capable to run NDILib
/// currently NDILib requires SSE4.2 instructions (see documentation). You can verify
/// a specific CPU against the library with a call to [`is_supported_CPU()`]
pub fn initialize() -> Result<(), NDIError> {
    if !unsafe { NDIlib_initialize() } {
        return Err(NDIError::NotSupported);
    };

    Ok(())
}

/// Destroy everything associated with the library
///
/// This is not actually required, but will end the libraries which might get
/// you slightly better performance in some cases.In general it is more "correct" to
/// call this although it is not required.
/// This will destroy everything associated with the library so use it with due caution.
pub unsafe fn cleanup() {
    NDIlib_destroy();
}

/// Recover whether the current CPU in the system is capable of running NDILib.
#[allow(non_snake_case)]
pub fn is_supported_CPU() -> bool {
    unsafe { NDIlib_is_supported_CPU() }
}
