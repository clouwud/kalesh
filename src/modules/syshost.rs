use libc::gethostname;
use std::ffi::CStr;

pub fn get_host() -> String {
    let mut buf = [0u8; 256];

    unsafe {
        if gethostname(buf.as_mut_ptr() as *mut _, buf.len()) == 0 {
            if let Ok(cstr) = CStr::from_bytes_until_nul(&buf) {
                return cstr.to_string_lossy().to_string();
            }
        }
    }

    "unknown".to_string()
}
