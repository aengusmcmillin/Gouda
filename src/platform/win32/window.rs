extern crate winapi;

use crate::window::{GameWindowImpl, WindowProps};
use crate::input::GameInput;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::_core::ptr::null_mut;
use winapi::um::winuser::{WNDCLASSW, WNDCLASSEXA, CS_HREDRAW, CS_VREDRAW, CS_OWNDC, MessageBoxA, RegisterClassW, CW_USEDEFAULT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, CreateWindowExW};
use winapi::shared::minwindef::{UINT};
use winapi::shared::windef::{HWND};
use winapi::um::xinput::{XINPUT_VIBRATION, XINPUT_STATE};
use std::ffi::{OsStr, CString};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{LRESULT, LPARAM, WPARAM};
use winapi::shared::windef::{POINT};
use self::winapi::um::libloaderapi::{LoadLibraryW, GetProcAddress};
use self::winapi::shared::minwindef::{__some_function, DWORD};
use std::mem::transmute;
use self::winapi::um::winuser::{MSG, DefWindowProcW, WS_SYSMENU, WS_MINIMIZEBOX, WS_CAPTION, ShowWindow, SW_SHOW, PeekMessageW, PM_REMOVE, TranslateMessage, DispatchMessageW, WM_CLOSE, WM_KEYDOWN, WM_KEYUP, WM_CHAR, WM_LBUTTONDOWN, AdjustWindowRect};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::dxgi::CreateDXGIFactory;

trait Empty {
    fn empty() -> Self;
}

impl Empty for MSG {
    fn empty() -> MSG {
        MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT::empty(),
        }
    }
}

impl Empty for POINT {
    fn empty() -> POINT {
        POINT { x: 0, y: 0 }
    }
}

pub struct Window {
    pub hwnd: HWND,
    props: WindowProps,
}

impl Window {
    pub fn new(props: WindowProps) -> Window {
        let window = create_window("GoudaWindowClass", props.title.as_str(), props.width as u32, props.height as u32).unwrap();
        unsafe {ShowWindow(window, SW_SHOW)};
        Self {
            hwnd: window,
            props,
        }
    }
}
impl GameWindowImpl for Window {
    fn capture_input(&mut self) -> GameInput {
        let mut msg = MSG::empty();
        while unsafe {PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0} {
            match msg.message {
                WM_KEYDOWN => {
                    println!("Keydown {}", msg.wParam as u8 as char);
                }
                WM_KEYUP => {

                }
                WM_CHAR => {

                }
                WM_LBUTTONDOWN => {

                }
                _ => {

                }
            }
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
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
        WM_CLOSE => {
            0
        }
        _ => {
            return DefWindowProcW(window, message, wparam, lparam);
        },
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

fn create_window(class_name: &str, title: &str, width: u32, height: u32) -> Option<HWND> {
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
                    WS_SYSMENU | WS_MINIMIZEBOX | WS_CAPTION,
                    200,
                    200,
                    width as i32,
                    height as i32,
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
