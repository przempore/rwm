use std::mem;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use windows::HRESULT;

use bindings::Windows::Win32::{
    Graphics::Dwm::DwmGetWindowAttribute,
    System::Diagnostics::Debug::{GetLastError, WIN32_ERROR},
    System::StationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
    System::SystemServices::{BOOL, FALSE, HINSTANCE, LRESULT, PSTR, PWSTR, TRUE},
    System::Threading::GetCurrentThreadId,
    UI::DisplayDevices::{POINT, RECT},
    UI::KeyboardAndMouseInput::GetAsyncKeyState,
    UI::WindowsAndMessaging::*,
};

use core::ffi::c_void;

pub struct EventInterceptor {
    key_pressed: Arc<AtomicBool>,
}

impl EventInterceptor {
    pub fn new() -> EventInterceptor {
        EventInterceptor {
            key_pressed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn register_events(&self) {
        thread::spawn(move || {
            let h_instance = HINSTANCE::default();
            let hook = unsafe {
                SetWindowsHookExW(WH_MOUSE_LL, Some(Self::on_mouse_event), h_instance, 0)
            };

            println!("hook: {:?}", hook);

            if hook.is_null() {
                println!("Can't set windows hook. Error: {:?}", unsafe {
                    GetLastError()
                });
            }

            unsafe { GetMessageW(&mut MSG::default(), HWND::default(), 0, 0) };

            unsafe {
                UnhookWindowsHookEx(hook);
            }
        });

        thread::spawn(move || {
            let h_instance = HINSTANCE::default();
            let hook = unsafe {
                SetWindowsHookExW(WH_KEYBOARD_LL, Some(Self::on_keyboard_event), h_instance, 0)
            };

            if hook.is_null() {
                println!("Can't set windows hook. Error: {:?}", unsafe {
                    GetLastError()
                });
            }

            let mut msg = MSG::default();
            let hwnd = HWND::default();
            unsafe { GetMessageW(&mut msg, hwnd, 0, 0) };
            println!("msg: {:?}", msg);
            println!("hwnd: {:?}", hwnd);

            unsafe {
                UnhookWindowsHookEx(hook);
            }
        });
    }

    extern "system" fn on_mouse_event(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code < 0 {
            return unsafe { CallNextHookEx(HHOOK::default(), code, w_param, l_param) };
        }

        fn get_cursor_position() -> Result<(i32, i32), WIN32_ERROR> {
            let mut p = POINT::default();
            if unsafe { GetCursorPos(&mut p) == FALSE } {
                return Err(unsafe { GetLastError() });
            }

            Ok((p.x, p.y))
        }

        match get_cursor_position() {
            Ok((x, y)) => {
                if w_param == WPARAM(WM_LBUTTONDOWN as usize) {
                    println!("Left mouse button pressed at position: <{},{}>", x, y);

                    if unsafe { GetAsyncKeyState(VK_LCONTROL as i32) != 0 } {
                        println!("Left control button pressed!");
                    }

                    // let is_key_pressed = key_pressed.clone();

                    // if is_key_pressed.load(Ordering::Relaxed) {
                    //     println!("Key pressed and button pressed!");
                    // }
                    // std::thread::sleep(Duration::from_millis(150));
                    // let hwnd = unsafe { GetForegroundWindow() };
                    // let title = get_window_title(hwnd);
                    // println!("{:?}", title);
                } else if w_param == WPARAM(WM_RBUTTONDOWN as usize) {
                    println!("Right mouse button pressed at position: <{},{}>", x, y);
                    // } else if w_param == WPARAM(WM_MBUTTONDOWN) {
                    //     list_all_windows();
                }
            }
            Err(err) => println!("Can't get mouse position. Error: {:?}", err),
        };

        unsafe { CallNextHookEx(HHOOK::default(), code, w_param, l_param) }
    }

    extern "system" fn on_keyboard_event(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code < 0 {
            return unsafe { CallNextHookEx(HHOOK::default(), code, w_param, l_param) };
        }

        // let is_key_pressed = KEY_PRESSED.clone();

        if w_param == WPARAM(WM_KEYDOWN as usize) {
            println!("Key pressed! code={:?}, l_param={:?}", code, l_param);
            // todo: get key code
            // let st = l_param.as_ptr() KBDLLHOOKSTRUCT;
            // is_key_pressed.store(true, Ordering::Relaxed);
        } else if w_param == WPARAM(WM_KEYUP as usize) {
            // is_key_pressed.store(false, Ordering::Relaxed);
            println!("Key up! code={:?}, l_param={:?}", code, l_param);
        }
        unsafe { CallNextHookEx(HHOOK::default(), code, w_param, l_param) }
    }
}

// static KEY_PRESSED: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

pub fn list_all_windows() {
    unsafe {
        EnumDesktopWindows(
            GetThreadDesktop(GetCurrentThreadId()),
            Some(enum_proc),
            LPARAM(0),
        );
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

fn get_window_rect(hwnd: HWND) -> Result<Rect, String> {
    let mut rect = RECT::default();
    let got_rect = unsafe { GetWindowRect(hwnd, &mut rect) == TRUE };

    if !got_rect {
        return Err(format!("Can't get rect."));
    }

    Ok(Rect {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    })
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    if !is_visible(hwnd) {
        return false;
    }

    if is_iconic(hwnd) {
        return false;
    }

    let window_title = get_window_title(hwnd);
    if let Err(_) = window_title {
        return false;
    }

    if is_core_window(hwnd) {
        return false;
    }

    if is_app_window(hwnd) {
        return true;
    }

    if is_tool_window(hwnd) {
        return false;
    }

    if is_no_activate(hwnd) {
        return false;
    }

    if has_i_task_list_deleted_property(hwnd) {
        return false;
    }

    if is_application_frame_window(hwnd) {
        if !has_appropriate_application_view_cloak_type(hwnd) {
            return false;
        };
        if let Ok(_) = window_title {
            return true;
        }

        return false;
    }

    let window_owner = get_window_owner(hwnd);
    if let Some(wo) = window_owner {
        if !is_visible(wo) {
            return false;
        }
    }

    true
}

fn is_visible(hwnd: HWND) -> bool {
    unsafe { IsWindowVisible(hwnd) == TRUE }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum WindowError {
    NotFound,
}

fn get_window_title(hwnd: HWND) -> Result<String, WindowError> {
    const SIZE: usize = 128;
    let mut buf = [0u16; SIZE];

    let title_name_len = unsafe { GetWindowTextW(hwnd, PWSTR(buf.as_mut_ptr()), SIZE as i32) };
    if title_name_len == 0 {
        return Err(WindowError::NotFound);
    }

    let txt: Vec<u8> = buf.iter().map(|&c| c as u8).collect();
    match String::from_utf8(txt.clone()) {
        Ok(name) => Ok(format!("{}", truncate(&name, title_name_len as usize))),
        Err(_) => Ok(String::from_utf8_lossy(&txt).into_owned()),
    }
}

fn is_core_window(hwnd: HWND) -> bool {
    get_class_name(hwnd) == "Windows.UI.Core.CoreWindow".to_string()
}

fn is_application_frame_window(hwnd: HWND) -> bool {
    get_class_name(hwnd) == "ApplicationFrameWindow".to_string()
}

fn has_appropriate_application_view_cloak_type(hwnd: HWND) -> bool {
    match is_cloaked(hwnd) {
        Ok(ok) => ok == 0,
        Err(_) => false,
    }
}

fn get_window_owner(hwnd: HWND) -> Option<HWND> {
    let handle = unsafe { GetWindow(hwnd, GW_OWNER) };

    if handle.is_null() {
        return None;
    }

    Some(handle)
}

fn is_iconic(hwnd: HWND) -> bool {
    unsafe { IsIconic(hwnd) == TRUE }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn get_class_name(hwnd: HWND) -> String {
    let mut class_name = String::new();
    const SIZE: usize = 128;
    let mut buf = [0u8; SIZE];
    let class_name_len = unsafe { GetClassNameA(hwnd, PSTR(buf.as_mut_ptr()), SIZE as i32) };
    if class_name_len > 0 {
        let txt = buf.iter().map(|&c| c).collect();
        class_name = String::from_utf8(txt).unwrap_or_else(|error| {
            println!("Windows title error: {}", error);
            "Incorrect window title!".to_string()
        });
        class_name = String::from(truncate(&class_name, class_name_len as usize));
    }
    return class_name;
}

fn is_app_window(hwnd: HWND) -> bool {
    let flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
    let flag = WINDOW_EX_STYLE::from(flag as u32);
    WS_EX_APPWINDOW == flag
}

fn is_tool_window(hwnd: HWND) -> bool {
    let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) as u32 };
    let mut ret = WINDOW_EX_STYLE::from(ex_style_flag) == WS_EX_TOOLWINDOW;
    let style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_STYLE) as u32 };
    ret |= WINDOW_EX_STYLE::from(style_flag) == WS_EX_TOOLWINDOW;

    ret
}

fn is_no_activate(hwnd: HWND) -> bool {
    let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) as u32 };
    WINDOW_EX_STYLE::from(ex_style_flag) == WS_EX_NOACTIVATE
}

fn has_i_task_list_deleted_property(hwnd: HWND) -> bool {
    let mut c_to_print = String::from("ITaskList_Deleted");
    let buf = PSTR(c_to_print.as_mut_ptr());
    unsafe { !GetPropA(hwnd, buf).is_null() }
}

fn is_cloaked(hwnd: HWND) -> Result<i32, String> {
    let mut pv_attribute = unsafe { mem::MaybeUninit::<i32>::zeroed().assume_init() };
    let (ret, pv_attribute) = unsafe {
        let ret = DwmGetWindowAttribute(
            hwnd,
            14u32,
            &mut pv_attribute as *mut _ as *mut c_void,
            mem::size_of::<u32>() as u32,
        );
        (ret, pv_attribute)
    };

    match ret {
        HRESULT(0) => Ok(pv_attribute),
        _ => Err(format!("Returned HRESULT: {:?}", ret)), // an invalid handle, or type size for the given attribute?
    }
}

extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if is_alt_tab_window(hwnd) {
        match get_window_title(hwnd) {
            Ok(title) => {
                let rect = get_window_rect(hwnd);
                println!("-> {}, rect: {:?}", title, rect)
            }
            Err(err) => {
                println!("error: {:?}", err)
            }
        };
    }

    TRUE
}
