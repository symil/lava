use std::vec::Vec;
use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct CharPP {
    c_strings: Vec<CString>,
    pointers: Vec<*const c_char>
}

impl CharPP {
    pub fn new(strings: &Vec<String>) -> CharPP {
        let c_strings : Vec<CString> = strings.into_iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
        let pointers : Vec<*const c_char> = (&c_strings).into_iter().map(|s| s.as_ptr()).collect();

        CharPP {
            c_strings: c_strings,
            pointers: pointers
        }
    }

    pub fn as_ptr(&self) -> *const *const c_char {
        self.pointers.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.pointers.len()
    }
}