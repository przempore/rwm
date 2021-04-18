mod tray_icon;
mod winapi_wrapper;

use tray_icon::show_tray_icon; // if you want to use tray_icon

fn main() -> Result<(), systray::Error> {
    show_tray_icon() // if you want to use tray_icon
}
