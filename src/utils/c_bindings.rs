#[allow(non_camel_case_types)]
pub type c_void = u8;

extern {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn calloc(nb_items: usize, size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
}