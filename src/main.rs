mod tray_icon;
mod winapi_wrapper;

// use tray_icon::show_tray_icon; // if you want to use tray_icon
use winapi_wrapper::list_all_windows;

fn main() -> Result<(), systray::Error> {
    list_all_windows();
    // show_tray_icon() // if you want to use tray_icon

    Ok(())
}
