use super::internal::bindings::*;
use super::*;
use std::ffi::CString;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct FindBuilder {
    show_local_sources: Option<bool>,
    groups: Option<String>,
    extra_ips: Option<String>,
}

impl FindBuilder {
    pub fn new() -> Self {
        Self {
            show_local_sources: None,
            groups: None,
            extra_ips: None,
        }
    }

    pub fn show_local_sources(mut self, show_local_sources: bool) -> Self {
        self.show_local_sources = Some(show_local_sources);
        self
    }
    pub fn groups(mut self, groups: String) -> Self {
        self.groups = Some(groups);
        self
    }

    pub fn extra_ips(mut self, extra_ips: String) -> Self {
        self.extra_ips = Some(extra_ips);
        self
    }

    pub fn build(self) -> Result<Find, String> {
        // from default c++ constructor in Processing.NDI.Find.h
        let mut settings = NDIlib_find_create_t {
            show_local_sources: true,
            p_groups: NULL as _,
            p_extra_ips: NULL as _,
        };

        if let Some(show_local_sources) = self.show_local_sources {
            settings.show_local_sources = show_local_sources;
        }

        if let Some(groups) = self.groups {
            let cstr = CString::new(groups).map_err(|x| x.to_string())?;
            settings.p_groups = cstr.as_ptr();
        }

        if let Some(extra_ips) = self.extra_ips {
            let cstr = CString::new(extra_ips).map_err(|x| x.to_string())?;
            settings.p_extra_ips = cstr.as_ptr();
        }

        Find::with_settings(settings)
    }
}

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

    pub fn with_settings(settings: NDIlib_find_create_t) -> Result<Self, String> {
        let p_instance = unsafe { NDIlib_find_create_v2(&settings) };
        if p_instance.is_null() {
            return Err("Failed to create new NDI Find instance.".to_string());
        };

        Ok(Self { p_instance })
    }

    pub fn current_sources(&self, timeout_ms: u128) -> Result<Vec<Source>, String> {
        let mut no_sources = 0;
        let mut p_sources: *const NDIlib_source_t = NULL as _;
        let start = Instant::now();
        while no_sources == 0 {
            // timeout if it takes an unreasonable amount of time
            if Instant::now().duration_since(start).as_millis() > timeout_ms {
                return Err("Timeout on finding NDI sources".to_string());
            }

            p_sources =
                unsafe { NDIlib_find_get_current_sources(self.p_instance, &mut no_sources) };
        }

        let mut sources: Vec<Source> = vec![];
        for _ in 0..no_sources {
            sources.push(Source::from_binding(unsafe { *p_sources }));
            p_sources = unsafe { p_sources.add(1) };
        }

        Ok(sources)
    }
}

impl Drop for Find {
    fn drop(&mut self) {
        unsafe { NDIlib_find_destroy(self.p_instance) };
    }
}
