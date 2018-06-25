use utils::void;

extern {
    pub fn malloc(size: usize) -> *mut void;
    pub fn calloc(count: usize, size: usize) -> *mut void;
    pub fn free(ptr: *mut void);
    pub fn memcpy(dst: *mut void, src: *const void, count: usize);
}