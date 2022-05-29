use super::*;
use std::{convert::TryFrom, ffi::CString, mem::MaybeUninit};

/// Builder struct for [`Send`]
#[derive(Debug, Clone)]
pub struct SendBuilder {
    ndi_name: Option<String>,
    groups: Option<String>,
    clock_video: Option<bool>,
    clock_audio: Option<bool>,
}

impl SendBuilder {
    /// Create new builder instance
    pub fn new() -> Self {
        Self {
            ndi_name: None,
            groups: None,
            clock_video: None,
            clock_audio: None,
        }
    }

    /// This is the name of the NDI source to create.
    ///
    /// This will be the name of the NDI source on the network.
    /// For instance, if your network machine name is called “MyMachine” and you
    /// specify this parameter as “My Video”, the NDI source on the network would be “MyMachine (My Video)”.
    pub fn ndi_name(mut self, ndi_name: String) -> Self {
        self.ndi_name = Some(ndi_name);
        self
    }

    /// Specify the groups that this NDI sender should place itself into.
    ///
    /// Groups are sets of NDI sources. Any source can be part of any
    /// number of groups, and groups are comma-separated. For instance
    /// "cameras,studio 1,10am show" would place a source in the three groups named.
    pub fn groups(mut self, groups: String) -> Self {
        self.groups = Some(groups);
        self
    }

    /// Specify whether video "clock" themself.
    ///
    /// When it is clocked, video frames added will be rate-limited to
    /// match the current framerate they are submitted at.
    /// In general, if you are submitting video and audio off a single thread you should only clock one of them
    /// If you are submitting audio and video of separate threads then having both clocked can be useful.
    /// A simplified view of the how works is that, when you submit a frame, it will
    /// keep track of the time the next frame would be required at. If you submit a
    /// frame before this time, the call will wait until that time. This ensures that, if
    /// you sit in a tight loop and render frames as fast as you can go, they will be
    /// clocked at the framerate that you desire.
    ///
    /// Note that combining clocked video and audio submission combined with
    /// asynchronous frame submission (see below) allows you to write very simple
    /// loops to render and submit NDI frames.
    pub fn clock_video(mut self, clock_video: bool) -> Self {
        self.clock_video = Some(clock_video);
        self
    }

    /// specify whether audio "clock" themself.
    ///
    /// See above in `clock_video`
    pub fn clock_audio(mut self, clock_audio: bool) -> Self {
        self.clock_audio = Some(clock_audio);
        self
    }

    /// Build the [`Send`] instance
    pub fn build(self) -> Result<Send, SendCreateError> {
        let mut settings = NDIlib_send_create_t {
            p_ndi_name: null(),
            p_groups: null(),
            clock_video: true,
            clock_audio: true,
        };
        
        let cstr_ndi_name: CString;
        let cstr_ndi_group: CString;

        if let Some(ndi_name) = self.ndi_name {
            cstr_ndi_name = CString::new(ndi_name).unwrap();
            settings.p_ndi_name = cstr_ndi_name.as_ptr();
        }

        if let Some(groups) = self.groups {
            cstr_ndi_group = CString::new(groups).unwrap();
            settings.p_groups = cstr_ndi_group.as_ptr();
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

/// A sender struct for sending NDI
pub struct Send {
    p_instance: Arc<OnDrop<NDIlib_send_instance_t>>,
}

impl Send {
    /// Create a new instance with default parameters
    ///
    /// It is recommended to use [`SendBuilder`] instead
    pub fn new() -> Result<Self, SendCreateError> {
        let p_instance = unsafe { NDIlib_send_create(null()) };

        if p_instance.is_null() {
            return Err(SendCreateError);
        }

        Ok(Self {
            p_instance: Arc::new(OnDrop::new(p_instance, |s| unsafe {
                NDIlib_send_destroy(s)
            })),
        })
    }

    fn with_settings(settings: NDIlib_send_create_t) -> Result<Self, SendCreateError> {
        let p_instance = unsafe { NDIlib_send_create(&settings) };

        if p_instance.is_null() {
            return Err(SendCreateError);
        }

        Ok(Self {
            p_instance: Arc::new(OnDrop::new(p_instance, |s| unsafe {
                NDIlib_send_destroy(s)
            })),
        })
    }

    /// Get the current tally
    ///
    /// the return value is whether Tally was actually updated or not
    pub fn get_tally(&self, tally: &mut Tally, timeout_ms: u32) -> bool {
        unsafe {
            let p_tally = *tally;
            let is_updated =
                NDIlib_send_get_tally(**self.p_instance, &mut p_tally.into(), timeout_ms);
            is_updated
        }
    }

    /// This allows you to receive metadata from the other end of the connection
    pub fn capture(&self, meta_data: &mut Option<MetaData>, timeout_ms: u32) -> FrameType {
        unsafe {
            let mut p_meta = if let Some(metadata) = meta_data {
                MaybeUninit::new(metadata.p_instance)
            } else {
                MaybeUninit::uninit()
            };
            let frametype = NDIlib_send_capture(**self.p_instance, p_meta.as_mut_ptr(), timeout_ms);

            *meta_data = Some(MetaData::from_binding_send(
                Arc::clone(&self.p_instance),
                p_meta.assume_init(),
            ));

            let res: FrameType = FrameType::try_from(frametype).unwrap();

            res
        }
    }

    /// Retrieve the source information for the given sender instance.
    pub fn get_source(&self) -> Source {
        let instance = unsafe { *NDIlib_send_get_source_name(**self.p_instance) };
        let parent = SourceParent::Send(Arc::clone(&self.p_instance));
        Source::from_binding(parent, instance)
    }

    /// This will add a metadata frame
    pub fn send_metadata(&self, metadata: &MetaData) {
        unsafe {
            NDIlib_send_send_metadata(**self.p_instance, &metadata.p_instance);
        }
    }

    /// This will add an audio frame
    pub fn send_audio(&self, audio_data: &AudioData) {
        unsafe {
            NDIlib_send_send_audio_v3(**self.p_instance, &audio_data.p_instance);
        }
    }

    /// This will add a video frame
    pub fn send_video(&self, video_data: &VideoData) {
        unsafe {
            NDIlib_send_send_video_v2(**self.p_instance, &video_data.p_instance);
        }
    }

    /// This will add a video frame and will return immediately, having scheduled the frame to be displayed.
    ///
    /// All processing and sending of the video will occur asynchronously. The memory accessed by NDIlib_video_frame_t
    /// cannot be freed or re-used by the caller until a synchronizing event has occurred. In general the API is better
    /// able to take advantage of asynchronous processing than you might be able to by simple having a separate thread
    /// to submit frames.
    ///
    /// This call is particularly beneficial when processing BGRA video since it allows any color conversion, compression
    /// and network sending to all be done on separate threads from your main rendering thread.
    ///
    /// Synchronizing events are :
    /// - a call to `send_video`
    /// - a call to `send_video_async` with another frame to be sent
    /// - a call to `send_video` with p_video_data=NULL
    /// - Dropping a [`Send`] instance
    pub fn send_video_async(&self, video_data: &VideoData) {
        unsafe {
            NDIlib_send_send_video_async_v2(**self.p_instance, &video_data.p_instance);
        }
    }

    /// Get the current number of receivers connected to this source.
    ///
    /// This can be used to avoid even rendering when nothing is connected to the video source.
    /// which can significantly improve the efficiency if you want to make a lot of sources available on the network.
    /// If you specify a timeout that is not 0 then it will wait until there are connections for this amount of time.
    pub fn get_no_connections(&self, timeout_ms: u32) -> u32 {
        unsafe { NDIlib_send_get_no_connections(**self.p_instance, timeout_ms) as _ }
    }

    // Free the buffers returned by capture for metadata
    // pub(crate) fn free_metadata(&self, metadata: &mut MetaData) {
    //     unsafe {
    //         NDIlib_send_free_metadata(*self.p_instance, &metadata.p_instance);
    //     }
    // }
}
