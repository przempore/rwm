use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::winuser::{GetPropA, GetWindow, GetWindowLongPtrA};

use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        windef::HWND,
    },
    um::winuser::{EnumDesktopWindows, GetClassNameA, GetWindowTextW, IsIconic, IsWindowVisible},
};

pub fn list_all_windows() {
    unsafe {
        // EnumWindows(Some(enum_proc), 0);
        EnumDesktopWindows(null_mut(), Some(enum_proc), 0);
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

    if get_window_title(hwnd) == None {
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

    if is_application_frame_window(hwnd) && !has_appropriate_application_view_cloak_type(hwnd) {
        if let Some(title) = get_window_title(hwnd) {
            println!("--- DEBUG --- | name: {}", title);
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

fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe {
        const SIZE: usize = 1024;
        let mut buf = [0u16; SIZE];
        let title_name_len = GetWindowTextW(hwnd, &mut buf[0], SIZE as i32);
        if title_name_len == 0 {
            return None;
        }
        let title_name = String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap();
        Some(String::from(truncate(&title_name, title_name_len as usize)))
    }
}

fn is_core_window(hwnd: HWND) -> bool {
    get_class_name(hwnd) == "Windows.UI.Core.CoreWindow".to_string()
}

fn is_application_frame_window(hwnd: HWND) -> bool {
    get_class_name(hwnd) == "ApplicationFrameWindow".to_string()
}

fn has_appropriate_application_view_cloak_type(_hwnd: HWND) -> bool {
    true
}

fn get_window_owner(hwnd: HWND) -> Option<HWND> {
    unsafe {
        const GW_OWNER: u32 = 4;
        let handle = GetWindow(hwnd, GW_OWNER);

        if handle.is_null() {
            return None;
        }

        Some(handle)
    }
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
    unsafe {
        const SIZE: usize = 1024;
        let mut buf = [0i8; SIZE];

        let class_name_len = GetClassNameA(hwnd, &mut buf[0], SIZE as i32);
        if class_name_len > 0 {
            class_name = String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap();
            class_name = String::from(truncate(&class_name, class_name_len as usize));
        }
    }
    return class_name;
}

fn is_app_window(hwnd: HWND) -> bool {
    const GWL_EXSTYLE: i32 = -20;
    const APPWINDOW: isize = 0x00040000;
    unsafe {
        let flag = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        flag == APPWINDOW
    }
}

fn is_tool_window(hwnd: HWND) -> bool {
    const GWL_EXSTYLE: i32 = -20;
    const GWL_STYLE: i32 = -16;
    const TOOLWINDOW: isize = 0x00000080;

    let mut ret: bool;
    unsafe {
        let ex_style_flag = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        ret = ex_style_flag == TOOLWINDOW;

        let style_flag = GetWindowLongPtrA(hwnd, GWL_STYLE);
        ret |= style_flag == TOOLWINDOW;
    }
    return ret;
}

fn is_no_activate(hwnd: HWND) -> bool {
    const NOACTIVATE: isize = 0x08000000;
    const GWL_EXSTYLE: i32 = -20;
    unsafe {
        let ex_style_flag = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        ex_style_flag == NOACTIVATE
    }
}

fn has_i_task_list_deleted_property(hwnd: HWND) -> bool {
    let c_to_print = CString::new("ITaskList_Deleted").expect("CString::new failed");
    unsafe { !GetPropA(hwnd, c_to_print.as_ptr()).is_null() }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if is_alt_tab_window(hwnd) {
        if let Some(win_title) = get_window_title(hwnd) {
            println!("{}", win_title);
        }
    }

    TRUE
}
