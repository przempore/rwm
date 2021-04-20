use std::mem;
use windows::HRESULT;

use bindings::Windows::Win32::{
    Dwm::DwmGetWindowAttribute,
    SystemServices::{GetCurrentThreadId, BOOL, PSTR, PWSTR, TRUE},
    WindowsAndMessaging::*,
    WindowsStationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
};

use core::ffi::c_void;

pub fn list_all_windows() {
    unsafe {
        EnumDesktopWindows(
            GetThreadDesktop(GetCurrentThreadId()),
            Some(enum_proc),
            LPARAM(0),
        );
    }
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    if !is_visible(hwnd) {
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
    let mut buf1 = [0u16; SIZE];
    let buf = PWSTR(buf1.as_mut_ptr());

    let title_name_len = unsafe { GetWindowTextW(hwnd, buf, SIZE as i32) };
    if title_name_len == 0 {
        return Err(WindowError::NotFound);
    }

    let txt: Vec<u8> = buf1.iter().map(|&c| c as u8).collect();
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
    let handle = unsafe { GetWindow(hwnd, GetWindow_uCmdFlags::GW_OWNER) };

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
    let mut buf1 = [0u8; SIZE];
    let buf = PSTR(buf1.as_mut_ptr());
    let class_name_len = unsafe { GetClassNameA(hwnd, buf, SIZE as i32) };
    if class_name_len > 0 {
        let txt = buf1.iter().map(|&c| c).collect();
        class_name = String::from_utf8(txt).unwrap_or_else(|error| {
            println!("Windows title error: {}", error);
            "Incorrect window title!".to_string()
        });
        class_name = String::from(truncate(&class_name, class_name_len as usize));
    }
    return class_name;
}

fn is_app_window(hwnd: HWND) -> bool {
    let flag = unsafe { GetWindowLongPtrA(hwnd, WINDOW_LONG_PTR_INDEX::GWL_EXSTYLE) };
    let flag = WINDOW_EX_STYLE::from(flag as u32);
    WINDOW_EX_STYLE::WS_EX_APPWINDOW == flag
}

fn is_tool_window(hwnd: HWND) -> bool {
    let ex_style_flag =
        unsafe { GetWindowLongPtrA(hwnd, WINDOW_LONG_PTR_INDEX::GWL_EXSTYLE) as u32 };
    let mut ret = WINDOW_EX_STYLE::from(ex_style_flag) == WINDOW_EX_STYLE::WS_EX_TOOLWINDOW;
    let style_flag = unsafe { GetWindowLongPtrA(hwnd, WINDOW_LONG_PTR_INDEX::GWL_STYLE) as u32 };
    ret |= WINDOW_EX_STYLE::from(style_flag) == WINDOW_EX_STYLE::WS_EX_TOOLWINDOW;

    ret
}

fn is_no_activate(hwnd: HWND) -> bool {
    let ex_style_flag =
        unsafe { GetWindowLongPtrA(hwnd, WINDOW_LONG_PTR_INDEX::GWL_EXSTYLE) as u32 };
    WINDOW_EX_STYLE::from(ex_style_flag) == WINDOW_EX_STYLE::WS_EX_NOACTIVATE
}

fn has_i_task_list_deleted_property(hwnd: HWND) -> bool {
    // let c_to_print = String::new("ITaskList_Deleted").expect("CString::new failed");
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

    // todo: what is wrong with it?
    match ret {
        HRESULT(0) => Ok(pv_attribute),
        _ => Err(format!("Returned HRESULT: {:?}", ret)), // an invalid handle, or type size for the given attribute?
    }
}

extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if is_alt_tab_window(hwnd) {
        match get_window_title(hwnd) {
            Ok(title) => println!("-> {}", title),
            Err(err) => println!("error: {:?}", err),
        };
    }

    TRUE
}
