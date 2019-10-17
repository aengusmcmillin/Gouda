extern crate winapi;

use crate::window::{GameWindowImpl, WindowProps};
use crate::input::GameInput;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winuser::{WNDCLASSW, CS_HREDRAW, CS_VREDRAW, CS_OWNDC, MessageBoxA, RegisterClassW, CW_USEDEFAULT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, CreateWindowExW};
use winapi::shared::minwindef::{UINT};
use winapi::shared::windef::{HWND};
use winapi::um::xinput::{XINPUT_VIBRATION, XINPUT_STATE};
use std::ffi::{OsStr, CString};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{LRESULT, LPARAM, WPARAM};
use self::winapi::um::libloaderapi::{LoadLibraryW, GetProcAddress};
use self::winapi::shared::minwindef::{__some_function, DWORD};
use std::mem::transmute;
use self::winapi::um::winuser::DefWindowProcW;

pub struct Window {
    hwnd: HWND,
    props: WindowProps,
}

impl Window {
    pub fn new(props: WindowProps) -> Window {
        win32_load_xinput();
        let window = create_window("GoudaWindowClass", props.title.as_str());
        Self {
            hwnd: window.unwrap(),
            props,
        }
    }
}
impl GameWindowImpl for Window {
    fn capture_input(&mut self) -> GameInput {
        return GameInput::new();
    }

    fn get_width(&self) -> usize {
        return self.props.width as usize;
    }

    fn get_height(&self) -> usize {
        return self.props.height as usize;
    }
}

unsafe extern "system" fn win32_handle_proc(
    window: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        _ => (DefWindowProcW(window, message, wparam, lparam)),
    }
}
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

type XInputGetStateFunc = unsafe extern "system" fn(DWORD, *mut XINPUT_STATE) -> DWORD;
type XInputSetStateFunc = unsafe extern "system" fn(DWORD, *mut XINPUT_VIBRATION) -> DWORD;
static mut OPT_XINPUT_GET_STATE_FUNC: Option<XInputGetStateFunc> = None;
static mut OPT_XINPUT_SET_STATE_FUNC: Option<XInputSetStateFunc> = None;

fn win32_load_xinput() {
    unsafe {
        let dll_14 = win32_string("xinput1_4.dll");
        let mut xinput_lib = LoadLibraryW(dll_14.as_ptr());

        if xinput_lib.is_null() {
            let dll_910 = win32_string("xinput9_1_0.dll");
            xinput_lib = LoadLibraryW(dll_910.as_ptr());
        }

        if xinput_lib.is_null() {
            let dll_13 = win32_string("xinput1_3.dll");
            xinput_lib = LoadLibraryW(dll_13.as_ptr());
        }

        if !xinput_lib.is_null() {
            let get_state_name_c = CString::new("XInputGetState").unwrap();
            let get_state_ptr = GetProcAddress(xinput_lib, get_state_name_c.as_ptr());
            OPT_XINPUT_GET_STATE_FUNC = Some(
                transmute::<*mut __some_function, XInputGetStateFunc>(get_state_ptr),
            );

            let set_state_name_c = CString::new("XInputSetState").unwrap();
            let set_state_ptr = GetProcAddress(xinput_lib, set_state_name_c.as_ptr());
            OPT_XINPUT_SET_STATE_FUNC = Some(
                transmute::<*mut __some_function, XInputSetStateFunc>(set_state_ptr),
            );
        }
    }
}
fn create_window(class_name: &str, title: &str) -> Option<HWND> {
    let class_name = win32_string(class_name);
    let title = win32_string(title);

    let handle_instance = unsafe { GetModuleHandleW(null_mut()) };

    let window_class = WNDCLASSW {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(win32_handle_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: handle_instance,
        lpszClassName: class_name.as_ptr(),
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
    };
    unsafe {
        match RegisterClassW(&window_class) {
            0 => {
                println!("Failed to register class");
                MessageBoxA(
                    0 as HWND,
                    b"Call to RegisterClassEx failed!\0".as_ptr() as *const i8,
                    b"Win32 Guided Tour\0".as_ptr() as *const i8,
                    0 as UINT,
                );
                None
            }
            _atom => {
                let window = CreateWindowExW(
                    0,
                    window_class.lpszClassName,
                    title.as_ptr(),
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    null_mut(),
                    null_mut(),
                    handle_instance,
                    null_mut(),
                );
                if window.is_null() {
                    println!("Window is null");
                    None
                } else {
                    Some(window)
                }
            }
        }
    }
}
