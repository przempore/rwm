#pragma once

#include <windows.h>

namespace winapi_connection {

class WindowController {
public:
  WindowController(HWND hWnd);

  bool has_title() const;
  bool is_visible() const;

private:
	HWND hWnd_;
};

} // namespace winapi_connection