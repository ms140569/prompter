extern crate libc;

use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use std::env;
use prompt::Prompt;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct ReadOnlySegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for ReadOnlySegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let path;
        
        match env::current_dir() {
            Err(err) => {
                eprintln!("Could not get current directory: {}", err);
                return (String::from("*err*"), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
            },
            Ok(p) => {
                path = p.display().to_string();
            }
        } 

        let mut result = String::new();

        if libc_access(&CString::new(path).unwrap(), 2) != 0 {
            result.push_str(&format!(" {} ", prompt.symbols.lock)); 
        }
        return (result, prompt.theme[READONLY_FG], prompt.theme[READONLY_BG], String::new(), prompt.theme[RESET]);
    }
}


fn libc_access(path: &CStr, mode: libc::c_int) -> libc::c_int {
    extern {
        fn access(path: *const c_char, mode: libc::c_int) -> libc::c_int;
    }

    unsafe {
        return access(path.as_ptr(), mode); }

}

 
