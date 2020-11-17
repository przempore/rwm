#include "window_control.hpp"

#include <string>

namespace winapi_connection {

WindowController::WindowController(HWND hWnd) {}

bool WindowController::has_title() const {
  const int length = GetWindowTextLength(hWnd_);
  char *buffer = new char[length + 1];
  GetWindowText(hWnd_, buffer, length + 1);
  const std::string title{buffer};

  return title.empty();
}

bool WindowController::is_visible() const { return IsWindowVisible(hWnd_); }

} // namespace winapi_connection
