struct VkRef<W> {
    value: W,
}

impl<W> VkRef<W> {
    fn new_vk_value<R>(ptr: *const R) -> Self
        where W : VkType<R>
    {
        let value = W::vk_from_raw(unsafe { &*ptr });

        Self {
            value: value
        }
    }

    pub fn new_array<T>(length: u32, ptr: *const T) -> Vec<T>
        where T : Copy
    {
        unsafe {
            let len = length as usize;
            let t_size = mem::size_of::<T>();
            let copy_ptr = malloc(len * t_size) as *mut T;

            for i in 0..len {
                let src_ptr = ptr.offset(i as isize);
                let dst_ptr = copy_ptr.offset(i as isize);
                memcpy(dst_ptr as *mut c_void, src_ptr as *mut c_void, t_size);
            }

            Vec::from_raw_parts(copy_ptr, len, len)
        }
    }
}

impl VkRef<String> {
    fn new_string(ptr: *const c_char) -> Self {
        let string = unsafe { String::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes().to_vec()) };

        Self {
            value: string
        }
    }
}

impl<W> Deref for VkRef<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.value
    }
}