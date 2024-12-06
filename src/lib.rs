use std::ffi::c_void;

/// See https://www.speex.org/docs/api/speex-api-reference/speex__echo_8h.html
pub struct Aec {
    state: *mut aec_rs_sys::SpeexEchoState,
}

impl Aec {
    pub fn new(frame_size: i32, filter_length: i32) -> Self {
        unsafe {
            let state = aec_rs_sys::speex_echo_state_init(frame_size, filter_length);
            return Self { state };
        };
    }

    pub fn cancel(&mut self, rec: &mut [i16], play: &mut [i16]) -> Vec<i16> {
        let mut out: Vec<i16> = vec![0; rec.len()];
        unsafe {
            aec_rs_sys::speex_echo_cancellation(
                self.state,
                rec.as_mut_ptr(),
                play.as_mut_ptr(),
                out.as_mut_ptr(),
            );
        };
        out
    }

    pub fn reset(&mut self) {
        unsafe {
            aec_rs_sys::speex_echo_state_reset(self.state);
        }
    }

    pub fn get_frame_size(&mut self) -> i32 {
        let mut frame_size: i32 = 0;
        unsafe {
            aec_rs_sys::speex_echo_ctl(
                self.state,
                aec_rs_sys::SPEEX_ECHO_GET_FRAME_SIZE as _,
                &mut frame_size as *mut i32 as *mut c_void,
            );
        };
        frame_size
    }

    pub fn get_sample_rate(&mut self) -> i32 {
        let mut sample_rate: i32 = 0;
        unsafe {
            aec_rs_sys::speex_echo_ctl(
                self.state,
                aec_rs_sys::SPEEX_ECHO_GET_SAMPLING_RATE as _,
                &mut sample_rate as *mut i32 as *mut c_void,
            );
        }
        sample_rate
    }

    pub fn set_sample_rate(&mut self, mut sample_rate: i32) {
        unsafe {
            aec_rs_sys::speex_echo_ctl(
                self.state,
                aec_rs_sys::SPEEX_ECHO_SET_SAMPLING_RATE as _,
                &mut sample_rate as *mut i32 as *mut c_void,
            );
        };
    }
}

impl Drop for Aec {
    fn drop(&mut self) {
        unsafe {
            aec_rs_sys::speex_echo_state_destroy(self.state);
        }
    }
}
