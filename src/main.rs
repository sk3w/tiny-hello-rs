#![no_main]
#![no_std]
#![windows_subsystem = "windows"]

use core::panic::PanicInfo;
use winapi::shared::ntdef::LPCSTR;
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION};

#[panic_handler]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    main();
}

#[no_mangle]
pub fn main() -> i32 {
    const TEXT: &'static str = "Hello, world!\0";
    const CAPTION: &'static str = "Message\0";
    unsafe {
        MessageBoxA(
            core::ptr::null_mut(),
            TEXT.as_ptr() as LPCSTR,
            CAPTION.as_ptr() as LPCSTR,
            MB_OK | MB_ICONINFORMATION 
        );
    }
    0
}