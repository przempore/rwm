fn main() {
    windows::build!(
        Windows::Win32::WindowsStationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
        Windows::Win32::Dwm::{
            DWMWINDOWATTRIBUTE,
            DwmGetWindowAttribute,
        },
        Windows::Win32::Com::HRESULT,
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::SystemServices::{GetCurrentThreadId, BOOL, TRUE, PWSTR},
    )
}
