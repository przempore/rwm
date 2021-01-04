use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::ptr::null_mut;
use winapi::um::winuser::{GetAncestor, GetLastActivePopup, GA_ROOTOWNER};

use libc::c_void;
use winapi::{
    shared::minwindef::DWORD, shared::minwindef::HINSTANCE, um::errhandlingapi::GetLastError,
};

use winapi::{
    shared::{
        minwindef::{BOOL, FALSE, LPARAM, TRUE},
        windef::HWND,
    },
    um::{
        processthreadsapi::OpenProcess,
        psapi::{EnumProcessModules, GetModuleBaseNameA},
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        winuser::{
            EnumDesktopWindows, EnumWindows, GetClassNameA, GetWindowTextW,
            GetWindowThreadProcessId, IsIconic, IsWindowVisible,
        },
    },
};

pub fn list_all_windows() {
    unsafe {
        // EnumWindows(Some(enum_proc), 0);
        EnumDesktopWindows(null_mut(), Some(enum_proc), 0);
    }
}

unsafe extern "system" fn print_window_thread_process_id(hwnd: HWND) {
    let mut process_id: u32 = 0;
    let thread_id = GetWindowThreadProcessId(hwnd, &mut process_id);
    print!("thread: {}, pid: {}", thread_id, process_id);
    print_process_name_and_id(process_id);
}

// fn get_alt_tab_info(hwnd: HWND) {
//     unsafe {
//         let mut pati: PALTTABINFO = null_mut();
//         const SIZE: u32 = 1024;
//         let mut buf = vec![0i8; SIZE as usize];
//         let is_success = GetAltTabInfoA(hwnd, -1, pati, &mut buf[0], SIZE);
//         if is_success == FALSE {
//             println!("Failed to GetAltTabInfoA");
//         }
//     }
// }

unsafe extern "system" fn print_process_name_and_id(process_id: u32) {
    let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
    if h_process.is_null() {
        return;
    }
    const SIZE: u32 = 1024;
    let h_mod: [*mut c_void; SIZE as usize] = [0 as *mut c_void; SIZE as usize];
    let mut cb_needed: DWORD = 0;
    let mut buf = vec![0i8; SIZE as usize];
    let instance: HINSTANCE = null_mut();
    if EnumProcessModules(
        h_process,
        h_mod.as_ptr() as *mut HINSTANCE,
        SIZE,
        &mut cb_needed,
    ) == TRUE
    {
        let index: usize = 0x0;
        let buff_size = GetModuleBaseNameA(h_process, instance, &mut buf[index], SIZE);
        print!(" | buff_size: {}", buff_size);
        if buff_size == 0 {
            println!("GetModuleBaseNameA failed");
        }
        println!(
            " | name: {}",
            String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap()
        );
    } else {
        println!("EnumProcessModules failed! LastError: {}", GetLastError());
    }
}

// algorithm from https://devblogs.microsoft.com/oldnewthing/20071008-00/?p=24863
unsafe extern "system" fn is_alt_tab_window_from_blog(hwnd: HWND) -> bool {
    let mut hwnd_walk = GetAncestor(hwnd, GA_ROOTOWNER);
    let mut hwnd_try: HWND;
    loop {
        hwnd_try = GetLastActivePopup(hwnd_walk);
        if hwnd_try == hwnd_walk {
            break;
        }
        if IsWindowVisible(hwnd_try) == TRUE {
            break;
        }
        hwnd_walk = hwnd_try;
    }

    hwnd_walk == hwnd
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    // todo: distinguish between active desktop
    /*
        if (IsAppWindow()) return true;
        if (IsToolWindow()) return false;
        if (IsNoActivate()) return false;
        if (!IsOwnerOrOwnerNotVisible()) return false;
        if (HasITaskListDeletedProperty()) return false;
        if (IsApplicationFrameWindow() && !HasAppropriateApplicationViewCloakType()) return false;
    */
    unsafe {
        if IsWindowVisible(hwnd) == FALSE {
            return false;
        }

        const SIZE: usize = 1024;
        let mut buf = [0u16; SIZE];
        if GetWindowTextW(hwnd, &mut buf[0], SIZE as i32) <= 0 {
            return false;
        };
    }

    if is_core_window(hwnd) {
        return false;
    }

    true
}

fn is_core_window(hwnd: HWND) -> bool {
    get_class_name(hwnd) == "Windows.UI.Core.CoreWindow"
}

fn get_class_name(hwnd: HWND) -> String {
    let mut class_name = String::new();
    unsafe {
        const SIZE: usize = 1024;
        let mut buf = [0i8; SIZE];

        if GetClassNameA(hwnd, &mut buf[0], SIZE as i32) > 0 {
            class_name = String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap()
        }
    }
    return class_name;
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    const SIZE: usize = 1024;
    let mut buf = [0u16; SIZE];
    if IsWindowVisible(hwnd) == FALSE {
        return TRUE;
    }
    if GetWindowTextW(hwnd, &mut buf[0], SIZE as i32) > 0 {
        let win_text = decode(&buf);
        if IsIconic(hwnd) == FALSE {
            print!(". {} | ", win_text);
            println!(
                "--- DEBUG --- | is_alt_tab_window: {}",
                is_alt_tab_window(hwnd)
            );
            print_window_thread_process_id(hwnd);
        } else {
            println!("--- {} is a minimalized window.", win_text);
        }
    }
    TRUE
}

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
