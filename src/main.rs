mod tray_icon;
mod winapi_wrapper;

use tray_icon::show_tray_icon;
use winapi_wrapper::EventInterceptor;

fn main() -> Result<(), systray::Error> {
    let event_interceptor = EventInterceptor::new();
    event_interceptor.register_events();

    show_tray_icon()
}
