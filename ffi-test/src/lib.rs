use std::ffi::CString;
use std::os::raw::c_char;
use rascam::*;

#[no_mangle]
pub extern fn string_from_rust() -> *const c_char {
    let s = CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[repr(C)]
pub struct ImageBuffer {
    img_ptr: *mut u8,
    len: u32,
}

#[no_mangle]
pub extern fn take_photo() -> *mut ImageBuffer {
 let info = info().unwrap();
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    println!("{}", info);
    let mut camera = SimpleCamera::new(info.cameras[0].clone()).unwrap();
    camera.activate().unwrap();
    let mut bytes = camera.take_one().unwrap();
    let len = bytes.len() as u32;
    let ret = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    let mut ib = ImageBuffer { img_ptr: ret, len};
    let ret = &mut ib as *mut ImageBuffer;
    std::mem::forget(ib);
    ret
}

fn image_taken() -> *const c_char {
    let s = CString::new("Image saved as image.jpg").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}
