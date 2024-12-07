use std::os::raw::c_void;

/// See https://www.speex.org/docs/api/speex-api-reference/speex__echo_8h.html
use aec_rs_sys::*;

pub struct Aec {
    echo_state: *mut SpeexEchoState,
    preprocess_state: *mut SpeexPreprocessState,
}

impl Aec {
    pub fn new(frame_size: usize, filter_length: i32, sample_rate: u32) -> Self {
        let echo_state = unsafe { speex_echo_state_init(frame_size as i32, filter_length) };
        let preprocess_state = unsafe {
            let den = speex_preprocess_state_init(frame_size as i32, sample_rate as _);
            speex_preprocess_ctl(
                den,
                SPEEX_PREPROCESS_SET_ECHO_STATE as _,
                echo_state as *mut c_void,
            );
            den
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
            speex_preprocess_run(self.preprocess_state, out_buffer.as_mut_ptr());
        }
    }
}

impl Drop for Aec {
    fn drop(&mut self) {
        unsafe {
            speex_echo_state_destroy(self.echo_state);
            speex_preprocess_state_destroy(self.preprocess_state);
        }
    }
}
