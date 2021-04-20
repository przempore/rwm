mod tray_icon;
mod winapi_wrapper;

use tray_icon::show_tray_icon;

fn main() -> Result<(), systray::Error> {
    show_tray_icon()
}
