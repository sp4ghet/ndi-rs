use super::internal::bindings::*;
use super::*;
use std::time::Instant;

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
