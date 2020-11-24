use std::{ptr::null_mut};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

use libc::c_void;
use winapi::{shared::minwindef::HINSTANCE, shared::minwindef::{DWORD, HMODULE}, um::errhandlingapi::GetLastError, um::psapi::GetModuleFileNameExA};

use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE, FALSE},
        windef::HWND,
    },
    um::{
        processthreadsapi::OpenProcess,
        psapi::{EnumProcessModules, GetModuleBaseNameA},
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        winuser::{EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible},
        handleapi::CloseHandle,
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
        println!("thread: {}, pid: {}", thread_id, process_id);
    }
    print_process_name_and_id(process_id);
}

fn print_process_name_and_id(process_id: u32) {
    unsafe {
        let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if !h_process.is_null() {
            let mut h_mod: [*mut c_void; 1024] = [0 as *mut c_void; 1024];
            let mut cb_needed: DWORD = 0;
            const SIZE: u32 = 1024;
            let mut buf = vec![0i8; SIZE as usize];
            let mut instance: HINSTANCE = null_mut();
            if EnumProcessModules(h_process, h_mod.as_ptr() as *mut HINSTANCE, SIZE, &mut cb_needed) == TRUE {
                let mut index: usize = 0x0;
                let buff_size = GetModuleBaseNameA(h_process, instance, &mut buf[index], SIZE);
                println!("buff_size: {}", buff_size);
                if buff_size == 0 {
                    println!("GetModuleBaseNameA failed");
                }
                println!("name: {}", String::from_utf8(buf.iter().map(|&c| c as u8).collect()).unwrap());
            } else {
                println!("EnumProcessModules failed! LastError: {}", GetLastError());
            }
        }
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let mut buf = [0u16; 1024];
    if GetWindowTextW(hwnd, &mut buf[0], 1024) > 0 {
        let win_text = decode(&buf);
        if IsWindowVisible(hwnd) == TRUE {
            let mut process_id: u32 = 0;
            let _thread_id = GetWindowThreadProcessId(hwnd, &mut process_id);
            // println!("{} thread: {}, pid: {}", win_text, thread_id, process_id);
            print!("{} | ", win_text);
            print_window_thread_process_id(hwnd);
        }
    }
    TRUE
}

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
