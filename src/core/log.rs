use std::ffi::{c_char, CString};
use std::sync::Mutex;

extern "C" {
    fn console_log();
}

lazy_static! {
    static ref LOG_STRING: Mutex<String> = Mutex::new(String::from("ABCDEF"));
}

pub fn log(s: &str) {
    {
        let mut log_string = LOG_STRING.lock().unwrap();
        *log_string = s.to_owned();
    }
    unsafe { console_log() };
}

#[no_mangle]
pub extern "C" fn get_string() -> *mut c_char {
    let log_string = LOG_STRING.lock().unwrap();
    let s = CString::new(log_string.clone()).unwrap();
    s.into_raw()
}
