use std::ffi::CString;
use std::mem;

use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, LPARAM, LPVOID, TRUE},
        windef::HWND,
    },
    um::{
        dwmapi::{DwmGetWindowAttribute, DWMWA_CLOAKED},
        processthreadsapi::GetCurrentThreadId,
        winuser::*,
    },
};

pub fn list_all_windows() {
    unsafe {
        EnumDesktopWindows(GetThreadDesktop(GetCurrentThreadId()), Some(enum_proc), 0);
    }
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    // todo: distinguish between active desktop
    /*
        if (IsApplicationFrameWindow() && !HasAppropriateApplicationViewCloakType()) return false;
    */
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
    const SIZE: usize = 1024;
    let mut buf = [0u16; SIZE];

    let title_name_len = unsafe { GetWindowTextW(hwnd, &mut buf[0], SIZE as i32) };
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
    const GW_OWNER: u32 = 4;
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
    const SIZE: usize = 1024;
    let mut buf = [0i8; SIZE];
    let class_name_len = unsafe { GetClassNameA(hwnd, &mut buf[0], SIZE as i32) };
    if class_name_len > 0 {
        let txt = buf.iter().map(|&c| c as u8).collect();
        class_name = String::from_utf8(txt).unwrap_or_else(|error| {
            println!("Windows title error: {}", error);
            "Incorrect window title!".to_string()
        });
        class_name = String::from(truncate(&class_name, class_name_len as usize));
    }
    return class_name;
}

fn is_app_window(hwnd: HWND) -> bool {
    const APPWINDOW: isize = 0x00040000;
    let flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
    flag == APPWINDOW
}

fn is_tool_window(hwnd: HWND) -> bool {
    const TOOLWINDOW: isize = 0x00000080;
    let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
    let mut ret = ex_style_flag == TOOLWINDOW;
    let style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_STYLE) };
    ret |= style_flag == TOOLWINDOW;

    ret
}

fn is_no_activate(hwnd: HWND) -> bool {
    const NOACTIVATE: isize = 0x08000000;
    let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
    ex_style_flag == NOACTIVATE
}

fn has_i_task_list_deleted_property(hwnd: HWND) -> bool {
    let c_to_print = CString::new("ITaskList_Deleted").expect("CString::new failed");
    unsafe { !GetPropA(hwnd, c_to_print.as_ptr()).is_null() }
}

fn is_cloaked(hwnd: HWND) -> Result<i32, String> {
    let mut pv_attribute = unsafe { mem::MaybeUninit::<i32>::zeroed().assume_init() };
    let (ret, pv_attribute) = unsafe {
        let ret = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut pv_attribute as *mut _ as LPVOID,
            mem::size_of::<i32>() as DWORD,
        );
        (ret, pv_attribute)
    };

    match ret {
        0 => Ok(pv_attribute),                              // Wrapped attribute.
        _ => Err(format!("Returned HRESULT: 0x{:x}", ret)), // an invalid handle, or type size for the given attribute?
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if is_alt_tab_window(hwnd) {
        match get_window_title(hwnd) {
            Ok(title) => println!("-> {}", title),
            Err(err) => println!("error: {:?}", err),
        };
    }

    TRUE
}
