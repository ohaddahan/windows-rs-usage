mod win32;

use crate::win32::helpers::{enum_windows, get_class_name};
use std::ffi::c_void;
use std::ptr::addr_of_mut;
use std::time::Duration;
use std::{ptr, thread};
use windows::core::imp::{ConstBuffer, GetLastError};
use windows::core::{s, w, GUID, HSTRING, PCSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Com::CoInitialize;
use windows::Win32::UI::Accessibility::ObjectFromLresult;
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameA, GetClassNameW, GetDesktopWindow, GetParent, GetWindowThreadProcessId,
    RegisterWindowMessageA, RegisterWindowMessageW, SendMessageTimeoutA, SendMessageTimeoutW,
    SMTO_ABORTIFHUNG, SMTO_NORMAL,
};

fn main() -> () {
    let timeout = 5_000;

    unsafe {
        CoInitialize(None).unwrap();
        let wm_html_getobject = RegisterWindowMessageW(w!("wm_html_getobject"));
        let guid_html_document = GUID::from_u128(0x626FC520_A41E_11CF_A731_00A0C9082637);
        let h = GetDesktopWindow();
        let handles: Vec<isize> =  enum_windows(h).into_iter().filter(|h|  {
            let mut pid = 0;
            GetWindowThreadProcessId(HWND(*h), Some(&mut pid));
            let cls = get_class_name(*h);
            cls.contains("Internet Explorer_Server")
        }).collect::<Vec<isize>>();

        for handle in handles.iter() {
            let mut response: *mut usize = &mut 0usize;
            let ret = SendMessageTimeoutW(
                HWND(*handle),
                wm_html_getobject,
                WPARAM::default(),
                LPARAM::default(),
                SMTO_ABORTIFHUNG,
                timeout,
                Option::from(response),
            );
            let error = GetLastError();
            println!("handle = {:?} | error = {:?} | ret = {:?} | response = {:?}| *response = {:?}",handle, error, ret, response, *response);
            if ret.0 == 0 {
                continue;
            }

            // let mut doc = ptr::null_mut();
            // let mut doc = ptr::null::<usize>() as *mut usize as *mut c_void;
            // let mut doc: *mut c_void = ptr::null_mut();
            let mut _doc: *mut c_void = ptr::null_mut();
            let mut doc = addr_of_mut!(_doc);
            let result = ObjectFromLresult(
                LRESULT(*response as isize),
                &guid_html_document,
                WPARAM::default(),
                doc,
            );
            let error = GetLastError();
            println!("handle = {:?} | error = {:?} | result = {:?} | doc = {:?}",handle, error, result, doc);
        }
    }
}
