use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" {
    fn yolo_zig_abi_version() -> u32;
    fn yolo_zig_target_os() -> *const c_char;
    fn yolo_zig_target_arch() -> *const c_char;
}

fn main() {
    unsafe {
        let os = CStr::from_ptr(yolo_zig_target_os()).to_string_lossy();
        let arch = CStr::from_ptr(yolo_zig_target_arch()).to_string_lossy();
        println!(
            "{{\"abi\":{},\"os\":\"{os}\",\"arch\":\"{arch}\"}}",
            yolo_zig_abi_version()
        );
    }
}
