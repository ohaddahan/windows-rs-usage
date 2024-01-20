use std::mem;
use std::ptr::addr_of_mut;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, GetClassNameW};

pub fn get_class_name(handle: isize) -> String {
    unsafe {
        let handle = HWND(handle);
        let mut wstr = [0u16; 256];
        let size = GetClassNameW(handle, &mut wstr) as usize;
        let string = String::from_utf16_lossy(&wstr[0..size]);
        string
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, param: LPARAM) -> BOOL {
    let h: isize = hwnd.0;
    let p: isize = param.0;
    let handles: &mut Vec<isize> = mem::transmute(p as *const u8);
    handles.push(h);
    BOOL(1)
}

pub fn enum_windows(handle: HWND) -> Vec<isize> {
    let mut handles: Vec<isize> = Vec::with_capacity(1_000);
    let pointer = addr_of_mut!(handles);
    unsafe {
        EnumChildWindows(handle, Some(enum_proc), LPARAM(pointer as isize));
    }
    handles
}
