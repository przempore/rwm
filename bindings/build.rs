fn main() {
    windows::build!(
        windows::win32::display_devices::RECT,
        windows::win32::windows_and_messaging::{GetWindowTextW, GetForegroundWindow, SetWindowPos, GetWindowRect}
    )
}
