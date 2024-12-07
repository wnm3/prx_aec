use std::os::raw::c_void;

/// See https://www.speex.org/docs/api/speex-api-reference/speex__echo_8h.html
use aec_rs_sys::*;

#[derive(Debug, Clone)]
pub struct AecConfig {
    pub frame_size: usize,
    pub filter_length: i32,
    pub sample_rate: u32,
    pub enable_preprocess: bool,
}

impl Default for AecConfig {
    /// Default for 16khz
    fn default() -> Self {
        Self {
            frame_size: 160,
            filter_length: 1600,
            sample_rate: 16000,
            enable_preprocess: true,
        }
    }
}

pub struct Aec {
    echo_state: *mut SpeexEchoState,
    preprocess_state: Option<*mut SpeexPreprocessState>,
}

impl Aec {
    pub fn new(config: &AecConfig) -> Self {
        let echo_state =
            unsafe { speex_echo_state_init(config.frame_size as i32, config.filter_length) };
        let preprocess_state = if config.enable_preprocess {
            unsafe {
                let den =
                    speex_preprocess_state_init(config.frame_size as i32, config.sample_rate as _);
                speex_preprocess_ctl(
                    den,
                    SPEEX_PREPROCESS_SET_ECHO_STATE as _,
                    echo_state as *mut c_void,
                );
                Some(den)
            }
        } else {
            None
        };

        Aec {
            echo_state,
            preprocess_state,
        }
    }

    pub fn cancel_echo(&self, rec_buffer: &[i16], echo_buffer: &[i16], out_buffer: &mut [i16]) {
        unsafe {
            speex_echo_cancellation(
                self.echo_state,
                rec_buffer.as_ptr(),
                echo_buffer.as_ptr(),
                out_buffer.as_mut_ptr(),
            );
            if let Some(preprocess_state) = self.preprocess_state {
                speex_preprocess_run(preprocess_state, out_buffer.as_mut_ptr());
            }
        }
    }
}

impl Drop for Aec {
    fn drop(&mut self) {
        unsafe {
            speex_echo_state_destroy(self.echo_state);
            if let Some(preprocess_state) = self.preprocess_state {
                speex_preprocess_state_destroy(preprocess_state);
            }
        }
    }
}
