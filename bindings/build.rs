fn main() {
    windows::build!(
         Windows::Win32::System::StationsAndDesktops::{EnumDesktopWindows, GetThreadDesktop},
         Windows::Win32::Graphics::Dwm::{DWMWINDOWATTRIBUTE, DwmGetWindowAttribute},
         Windows::Win32::Foundation::{BOOL, HWND, HINSTANCE, LRESULT, POINT, PSTR, PWSTR, RECT},
         Windows::Win32::UI::WindowsAndMessaging::*,
         Windows::Win32::System::Threading::GetCurrentThreadId,
         Windows::Win32::System::Diagnostics::Debug::{GetLastError, WIN32_ERROR},
         Windows::Win32::UI::KeyboardAndMouseInput::{GetAsyncKeyState, GetKeyState},
    )
}
