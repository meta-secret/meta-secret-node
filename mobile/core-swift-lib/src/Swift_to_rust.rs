// use std::ffi::c_void;
// use std::ops::Deref;
use std::str;
use std::slice;

type SizeT = usize;

/*
#[repr(C)]
pub struct SwiftObject {
    user: *mut c_void,
    destroy: extern fn(user: *mut c_void),
    callback_with_int_arg: extern fn(user: *mut c_void, arg: i32),
}

unsafe impl Send for SwiftObject {}

struct SwiftObjectWrapper(SwiftObject);

impl Deref for SwiftObjectWrapper {
    type Target = SwiftObject;

    fn deref(&self) -> &SwiftObject {
        &self.0
    }
}

impl Drop for SwiftObjectWrapper {
    fn drop(&mut self) {
        (self.destroy)(self.user);
    }
}

#[no_mangle]
pub unsafe extern fn give_object_to_rust(obj: SwiftObject) {
    (obj.callback_with_int_arg)(obj.user, 10);
}
*/


#[no_mangle]
pub extern fn utf8_bytes_to_rust(bytes: *const u8, len: SizeT) {
    let byte_slice = unsafe { slice::from_raw_parts(bytes, len as usize) };
    print_byte_slice_as_utf8(byte_slice);
}

fn print_byte_slice_as_utf8(bytes: &[u8]) {
    match str::from_utf8(bytes) {
        Ok(s)    => println!("## got {}", s),
        Err(err) => println!("invalid UTF-8 data: {}", err),
    }
}