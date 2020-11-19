use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        windef::HWND,
    },
    um::winuser::{EnumWindows, GetWindowTextW},
};

fn show_tray_icon() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file(
        &"C:\\Users\\przem\\Projects\\rust\\systray-rs\\resources\\rust.ico".to_string(),
    );
    app.set_tooltip(&"Whatever".to_string());

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

fn main() -> Result<(), systray::Error> {
    unsafe {
        EnumWindows(Some(enum_proc), 0);
    }
    show_tray_icon()
}

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let mut buf = [0u16; 1024];
    if GetWindowTextW(hwnd, &mut buf[0], 1024) > 0 {
        let win_text = decode(&buf);
        println!("{}", win_text);
    }
    TRUE
}

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}
