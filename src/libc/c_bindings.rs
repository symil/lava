use libc::c_void;

extern {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn calloc(count: usize, size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn memcpy(dst: *mut c_void, src: *const c_void, count: usize);
}