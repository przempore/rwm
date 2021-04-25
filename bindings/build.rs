fn main() {
    windows::build!(
        Windows::Win32::WindowsStationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
        Windows::Win32::Dwm::{ DWMWINDOWATTRIBUTE, DwmGetWindowAttribute},
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::SystemServices::{GetCurrentThreadId, BOOL, FALSE, TRUE, PWSTR, LRESULT, HINSTANCE},
        Windows::Win32::DisplayDevices::POINT,
        Windows::Win32::Debug::GetLastError,
    )
}
