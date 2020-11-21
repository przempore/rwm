use std::char::{decode_utf16, REPLACEMENT_CHARACTER};

use winapi::shared::minwindef::HINSTANCE;

use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, LPARAM, TRUE},
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
        println!("thread: {}, pid: {}", thread_id, process_id);
    }
    // print_process_name_and_id(process_id);
}

// fn print_process_name_and_id(process_id: u32) {
//     let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
//     // println!("{}", GetLastError());

//     let mut buffer: [*mut c_void; 10] = [0 as *mut c_void; 10];
//     // WaitForInputIdle((*PI).hProcess as *mut c_void, -1);
//     let result = EnumProcessModules(
//         h_process as *mut c_void,
//         buffer.as_ptr() as *mut c_void,
//         10,
//         null_mut(),
//     );

//     println!(
//         "EnumProcessModules([...]) = {} - {}",
//         result,
//         GetLastError()
//     );

//     let mut index: usize = 0x0;
//     let mut modname: [u8; 1024] = [0; 1024];
//     while (transmute::<*mut c_void, u32>(buffer[index]) != 0x0) {
//         GetModuleFileNameExA(
//             (*PI).hProcess as *mut c_void,
//             buffer[index],
//             modname.as_ptr() as *mut c_void,
//             1024,
//         );
//         println!("module: {}", std::str::from_utf8_unchecked(&modname));
//         modname = [0; 1024];
//         index += 1;
//     }
// }

// fn print_process_name_and_id(process_id: u32) {
//     unsafe {
//         let h_process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
//         if !h_process.is_null() {
//             let mut h_mod: HINSTANCE;
//             let mut cb_needed: DWORD = 0;
//             const SIZE: u32 = 1024;
//             // let mut buf = [0i8; SIZE as usize];
//             let mut buf = vec![0i8; SIZE as usize];
//             let mut buffer: [*mut c_void; 10] = [0 as *mut c_void; 10];
//             if EnumProcessModules(h_process, &mut h_mod, SIZE, &mut cb_needed) == TRUE {
//                 let mut index: usize = 0x0;
//                 if GetModuleBaseNameA(h_process, h_mod, buffer[index], SIZE) > 0 {
//                     // {
//                     //     let mut v = std::mem::ManuallyDrop::new(buf);

//                     //     // then, pick apart the existing Vec
//                     //     let p = v.as_mut_ptr();
//                     //     let len = v.len();
//                     //     let cap = v.capacity();

//                     //     // finally, adopt the data into a new Vec
//                     //     let converted_buf = Vec::from_raw_parts(p as *mut u8, len, cap);
//                     //     let sparkle_heart = std::str::from_utf8(&converted_buf).unwrap();
//                     //     println!("sparkle_heart: {}", sparkle_heart);
//                     // }
//                 } else {
//                     println!("GetModuleBaseNameA failed");
//                 }
//             }
//         }
//     }
// }

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
