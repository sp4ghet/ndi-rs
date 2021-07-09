use std::convert::TryFrom;
use std::ffi::CString;

use super::internal::bindings::*;
use super::*;

#[derive(Debug, Clone)]
pub struct SendBuilder {
    ndi_name: Option<String>,
    groups: Option<String>,
    clock_video: Option<bool>,
    clock_audio: Option<bool>,
}

impl SendBuilder {
    pub fn new() -> Self {
        Self {
            ndi_name: None,
            groups: None,
            clock_video: None,
            clock_audio: None,
        }
    }

    pub fn ndi_name(mut self, ndi_name: String) -> Self {
        self.ndi_name = Some(ndi_name);
        self
    }

    pub fn groups(mut self, groups: String) -> Self {
        self.groups = Some(groups);
        self
    }

    pub fn clock_video(mut self, clock_video: bool) -> Self {
        self.clock_video = Some(clock_video);
        self
    }

    pub fn clock_audio(mut self, clock_audio: bool) -> Self {
        self.clock_audio = Some(clock_audio);
        self
    }

    pub fn build(self) -> Result<Send, String> {
        let mut settings = NDIlib_send_create_t {
            p_ndi_name: NULL as _,
            p_groups: NULL as _,
            clock_video: true,
            clock_audio: true,
        };

        if let Some(ndi_name) = self.ndi_name {
            let cstr = CString::new(ndi_name).map_err(|e| e.to_string())?;
            settings.p_ndi_name = cstr.as_ptr();
        }

        if let Some(groups) = self.groups {
            let cstr = CString::new(groups).map_err(|e| e.to_string())?;
            settings.p_groups = cstr.as_ptr();
        }

        if let Some(clock_video) = self.clock_video {
            settings.clock_video = clock_video;
        }

        if let Some(clock_audio) = self.clock_audio {
            settings.clock_audio = clock_audio;
        }

        Send::with_settings(settings)
    }
}

pub struct Send {
    p_instance: NDIlib_send_instance_t,
}

impl Send {
    pub fn new() -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_send_create(NULL as _) };

        if p_instance.is_null() {
            return Err("Failed to create NDI Send instance".to_string());
        }

        Ok(Self { p_instance })
    }
    pub fn with_settings(settings: NDIlib_send_create_t) -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_send_create(&settings) };

        if p_instance.is_null() {
            return Err("Failed to create NDI Send instance".to_string());
        }

        Ok(Self { p_instance })
    }

    /// Get the current tally
    ///
    /// the return value is whether Tally was actually updated or not
    pub fn get_tally(&self, tally: &mut Tally, timeout_ms: u32) -> bool {
        unsafe {
            let is_updated = NDIlib_send_get_tally(self.p_instance, tally, timeout_ms);
            is_updated
        }
    }

    pub fn capture(&self, metadata: &mut MetaData, timeout_ms: u32) -> FrameType {
        unsafe {
            let frametype =
                NDIlib_send_capture(self.p_instance, &mut metadata.p_instance, timeout_ms);

            let res: FrameType = FrameType::try_from(frametype).unwrap();

            res
        }
    }

    pub fn get_source(&self) -> Source {
        let instance = unsafe { *NDIlib_send_get_source_name(self.p_instance) };
        Source::from_binding(instance)
    }

    pub fn send_metadata(&self, metadata: &MetaData) {
        unsafe {
            NDIlib_send_send_metadata(self.p_instance, &metadata.p_instance);
        }
    }

    pub fn send_audio(&self, audio_data: &AudioData) {
        unsafe {
            NDIlib_send_send_audio_v3(self.p_instance, &audio_data.p_instance);
        }
    }

    pub fn send_video(&self, video_data: &VideoData) {
        unsafe {
            NDIlib_send_send_video_v2(self.p_instance, &video_data.p_instance);
        }
    }

    pub fn send_video_async(&self, video_data: &VideoData) {
        unsafe {
            NDIlib_send_send_video_async_v2(self.p_instance, &video_data.p_instance);
        }
    }

    pub fn get_no_connections(&self, timeout_ms: u32) -> u32 {
        unsafe { NDIlib_send_get_no_connections(self.p_instance, timeout_ms) as _ }
    }

    pub fn free_metadata(&self, metadata: MetaData) {
        unsafe {
            NDIlib_send_free_metadata(self.p_instance, &metadata.p_instance);
        }
    }
}

impl Drop for Send {
    fn drop(&mut self) {
        unsafe {
            NDIlib_send_destroy(self.p_instance);
        }
    }
}
