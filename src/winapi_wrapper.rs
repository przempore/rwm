use winapi::um::winuser::{GA_ROOTOWNER, GetAncestor, GetLastActivePopup};
use std::{ptr::null_mut};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

use libc::c_void;
use winapi::{shared::minwindef::HINSTANCE,
    shared::minwindef::{DWORD},
    um::errhandlingapi::GetLastError};

use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        windef::HWND,
    },
    um::{
        processthreadsapi::OpenProcess,
        psapi::{EnumProcessModules, GetModuleBaseNameA},
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        winuser::{EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible},
    },
};

pub fn list_all_windows() {
    unsafe {
        EnumWindows(Some(enum_proc), 0);
    }
}

fn print_window_thread_process_id(hwnd: HWND) {
    let mut process_id: u32 = 0;
    unsafe {
        let thread_id = GetWindowThreadProcessId(hwnd, &mut process_id);
        print!("thread: {}, pid: {}", thread_id, process_id);
    }
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

fn print_process_name_and_id(process_id: u32) {
    unsafe {
        let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if !h_process.is_null() {
            let h_mod: [*mut c_void; 1024] = [0 as *mut c_void; 1024];
            let mut cb_needed: DWORD = 0;
            const SIZE: u32 = 1024;
            let mut buf = vec![0i8; SIZE as usize];
            let instance: HINSTANCE = null_mut();
            if EnumProcessModules(h_process, h_mod.as_ptr() as *mut HINSTANCE, SIZE, &mut cb_needed) == TRUE {
                let index: usize = 0x0;
                let buff_size = GetModuleBaseNameA(h_process, instance, &mut buf[index], SIZE);
                print!(" | buff_size: {}", buff_size);
                if buff_size == 0 {
                    println!("GetModuleBaseNameA failed");
                }
                println!(" | name: {}", String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap());
            } else {
                println!("EnumProcessModules failed! LastError: {}", GetLastError());
            }
        }
    }
}

fn is_alt_tab_window(hwnd: HWND) -> bool {
    unsafe {
        let mut hwnd_walk = GetAncestor(hwnd, GA_ROOTOWNER);
        let mut hwnd_try : HWND;
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

        hwnd_try == hwnd
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let mut buf = [0u16; 1024];
    if GetWindowTextW(hwnd, &mut buf[0], 1024) > 0 {
        let win_text = decode(&buf);
        if IsWindowVisible(hwnd) == TRUE {
            if is_alt_tab_window(hwnd) {
                let mut process_id: u32 = 0;
                let _thread_id = GetWindowThreadProcessId(hwnd, &mut process_id);
                // println!("{} thread: {}, pid: {}", win_text, thread_id, process_id);
                print!("{} | ", win_text);
                print_window_thread_process_id(hwnd);
            } else {
                println!("--- {} is not an alt tab window", win_text);
            }
        }
    }
    TRUE
}

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
