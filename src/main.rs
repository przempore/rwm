mod tray_icon;
mod winapi_wrapper;

use tray_icon::show_tray_icon;
use winapi_wrapper::register_keyboard_clicks;
use winapi_wrapper::register_mouse_clicks;

fn main() -> Result<(), systray::Error> {
    register_mouse_clicks();
    register_keyboard_clicks();
    show_tray_icon()
}
