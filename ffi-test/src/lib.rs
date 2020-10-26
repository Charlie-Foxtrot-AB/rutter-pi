use std::ffi::CString;
use std::os::raw::c_char;
use rascam::SimpleCamera;

#[no_mangle]
pub extern fn string_from_rust() -> *const c_char {
    let s = CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[no_mangle]
pub extern fn take_photo() -> &'static [u8] {
    [0u8;100] 
}

#[no_mangle]
pub extern fn take_photo_and_write_to_disk() -> (&'static [u8], u32) {
 let info = info().unwrap();
    if info.cameras.len() < 1 {
        println!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    println!("{}", info);
    let mut camera = SimpleCamera::new(&info.cameras[0]).unwrap();
    camera.activate().unwrap();
    let bytes = camera.take_one().unwrap();
    let ret_arr_len = bytes.len();
    let ret_arr_slice = bytes.as_slice();
    (ret_arr_slice, ret_arr_len)
    /*File::create("/home/pi/image_from_flutter.jpg").unwrap().write_all(&b).unwrap();
    image_taken()*/
}

fn image_taken() -> *const c_char {
    let s = CString::new("Image saved as image.jpg").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}