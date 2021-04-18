use std::ffi::CString;
use std::mem;

use bindings::Windows::Win32::{
    Com::HRESULT,
    Dwm::{DwmGetWindowAttribute, DWMWINDOWATTRIBUTE},
    SystemServices::{GetCurrentThreadId, BOOL, PSTR, PWSTR, TRUE},
    WindowsAndMessaging::*,
    WindowsStationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
};

use core::ffi::c_void;

// use winapi::{
//     shared::{
//         minwindef::{BOOL, DWORD, LPARAM, LPVOID, TRUE},
//         windef::HWND,
//     },
//     um::{
//         dwmapi::{DwmGetWindowAttribute, DWMWA_CLOAKED},
//         processthreadsapi::GetCurrentThreadId,
//         winuser::*,
//     },
// };

// use std::io::{stdin, stdout, Write};

// fn pause() {
//     let mut stdout = stdout();
//     stdout.write(b"\nPress Enter to continue...").unwrap();
//     stdout.flush().unwrap();
//     stdin().read_line(&mut String::new()).unwrap();
// }

pub fn list_all_windows() {
    unsafe {
        EnumDesktopWindows(
            GetThreadDesktop(GetCurrentThreadId()),
            Some(enum_proc),
            LPARAM(0),
        );
    }

    // pause();
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    // todo: distinguish between active desktop
    /*
        if (IsApplicationFrameWindow() && !HasAppropriateApplicationViewCloakType()) return false;
    */
    if !is_visible(hwnd) {
        return false;
    }

    // let window_title = get_window_title(hwnd);
    // if let Err(_) = window_title {
    //     return false;
    // }

    // if is_core_window(hwnd) {
    //     return false;
    // }

    // if is_app_window(hwnd) {
    //     return true;
    // }

    // if is_tool_window(hwnd) {
    //     return false;
    // }

    // if is_no_activate(hwnd) {
    //     return false;
    // }

    // if has_i_task_list_deleted_property(hwnd) {
    //     return false;
    // }

    // if is_application_frame_window(hwnd) {
    //     if !has_appropriate_application_view_cloak_type(hwnd) {
    //         return false;
    //     };
    //     if let Ok(_) = window_title {
    //         return true;
    //     }

    //     return false;
    // }

    // let window_owner = get_window_owner(hwnd);
    // if let Some(wo) = window_owner {
    //     if !is_visible(wo) {
    //         return false;
    //     }
    // }

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
    // let mut buf: PWSTR = PWSTR::NULL;
    let buf = PWSTR::default();

    let title_name_len = unsafe { GetWindowTextW(hwnd, buf, SIZE as i32) };
    println!("buf: {:?}, title_name_len: {:?}", buf, title_name_len);
    if title_name_len == 0 {
        return Err(WindowError::NotFound);
    }

    Ok(String::from("sucess"))
    // let txt: Vec<u8> = buf.iter().map(|&c| c as u8).collect();
    // match String::from_utf8(txt.clone()) {
    //     Ok(name) => Ok(format!("{}", truncate(&name, title_name_len as usize))),
    //     Err(_) => Ok(String::from_utf8_lossy(&txt).into_owned()),
    // }
}

// fn is_core_window(hwnd: HWND) -> bool {
//     get_class_name(hwnd) == "Windows.UI.Core.CoreWindow".to_string()
// }

// fn is_application_frame_window(hwnd: HWND) -> bool {
//     get_class_name(hwnd) == "ApplicationFrameWindow".to_string()
// }

// fn has_appropriate_application_view_cloak_type(hwnd: HWND) -> bool {
//     match is_cloaked(hwnd) {
//         Ok(ok) => ok == 0,
//         Err(_) => false,
//     }
// }

// fn get_window_owner(hwnd: HWND) -> Option<HWND> {
//     let handle = unsafe { GetWindow(hwnd, GetWindow_uCmdFlags::GW_OWNER) };

//     if handle.is_null() {
//         return None;
//     }

//     Some(handle)
// }

// fn is_iconic(hwnd: HWND) -> bool {
//     unsafe { IsIconic(hwnd) == TRUE }
// }

// fn truncate(s: &str, max_chars: usize) -> &str {
//     match s.char_indices().nth(max_chars) {
//         None => s,
//         Some((idx, _)) => &s[..idx],
//     }
// }

// fn get_class_name(hwnd: HWND) -> String {
//     let mut class_name = String::new();
//     const SIZE: usize = 1024;
//     let mut buf = [0i8; SIZE];
//     let class_name_len = unsafe { GetClassNameA(hwnd, &mut buf[0], SIZE as i32) };
//     if class_name_len > 0 {
//         let txt = buf.iter().map(|&c| c as u8).collect();
//         class_name = String::from_utf8(txt).unwrap_or_else(|error| {
//             println!("Windows title error: {}", error);
//             "Incorrect window title!".to_string()
//         });
//         class_name = String::from(truncate(&class_name, class_name_len as usize));
//     }
//     return class_name;
// }

// fn is_app_window(hwnd: HWND) -> bool {
//     let flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
//     flag == WS_EX_APPWINDOW as isize
// }

// fn is_tool_window(hwnd: HWND) -> bool {
//     let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
//     let mut ret = ex_style_flag == WS_EX_TOOLWINDOW as isize;
//     let style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_STYLE) };
//     ret |= style_flag == WS_EX_TOOLWINDOW as isize;

//     ret
// }

// fn is_no_activate(hwnd: HWND) -> bool {
//     let ex_style_flag = unsafe { GetWindowLongPtrA(hwnd, GWL_EXSTYLE) };
//     // ex_style_flag == WS_EX_NOACTIVATE as isize
//     ex_style_flag == MA_NOACTIVATE as isize
// }

// fn has_i_task_list_deleted_property(hwnd: HWND) -> bool {
//     // let c_to_print = CString::new("ITaskList_Deleted").expect("CString::new failed");
//     let c_to_print: PSTR = PSTR::from::<&str>("ITaskList_Deleted");
//     unsafe { !GetPropA(hwnd, c_to_print).is_null() }
// }

// fn is_cloaked(hwnd: HWND) -> Result<i32, String> {
//     let mut pv_attribute = unsafe { mem::MaybeUninit::<i32>::zeroed().assume_init() };
//     let dwmwa_cloaked = DWMWINDOWATTRIBUTE::DWMWA_CLOAKED;
//     let (ret, pv_attribute) = unsafe {
//         let ret = DwmGetWindowAttribute(
//             hwnd,
//             14u32,
//             &mut pv_attribute as *mut _ as *mut c_void,
//             mem::size_of::<u32>() as u32,
//         );
//         (ret, pv_attribute)
//     };

//     println!("{:?}", ret);

//     Ok(pv_attribute)

//     // match ret {
//     //     0 => Ok(pv_attribute),
//     //     // _ => Err(format!("Returned HRESULT: 0x{:x}", ret)), // an invalid handle, or type size for the given attribute?
//     //     _ => Err(format!("Returned HRESULT: 0x{:?}", ret)), // an invalid handle, or type size for the given attribute?
//     // }
// }

extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if is_alt_tab_window(hwnd) {
        match get_window_title(hwnd) {
            Ok(title) => println!("-> {}", title),
            Err(err) => println!("error: {:?}", err),
        };
    }

    TRUE
}
