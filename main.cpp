#include <Windows.h>
#include <psapi.h>

#include <QAction>
#include <QApplication>
#include <QDir>
#include <QFileInfo>
#include <QMenu>
#include <QMessageBox>
#include <QQmlApplicationEngine>
#include <QSystemTrayIcon>

#include <iostream>
#include <string>

WINDOWPLACEMENT getWindowPlacement(const HWND &hWnd) {
  WINDOWPLACEMENT wpl;
  if (!GetWindowPlacement(hWnd, &wpl)) {
    std::cout << "GetWindowPlacement Failed!\n";
    return {};
  }

  return wpl;
}

void PrintProcessNameAndID(DWORD processID) {
  TCHAR szProcessName[MAX_PATH] = TEXT("<unknown>");

  // Get a handle to the process.

  HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                                FALSE, processID);

  // Get the process name.

  if (NULL != hProcess) {
    HMODULE hMod;
    DWORD cbNeeded;

    if (EnumProcessModules(hProcess, &hMod, sizeof(hMod), &cbNeeded)) {
      GetModuleBaseName(hProcess, hMod, szProcessName,
                        sizeof(szProcessName) / sizeof(TCHAR));
    }
  }

  // Print the process name and identifier.
  std::cout << szProcessName << " (PID: " << processID << ")" << std::endl;

  CloseHandle(hProcess);
}

void setWindowPos(HWND &hWnd) {
  auto width = (GetSystemMetrics(SM_CXSCREEN) * 2) / 7;
  auto height = width / 4;
  SetWindowPos(hWnd, HWND_TOP, 0, 0, width, height, SWP_SHOWWINDOW);
  if (!BringWindowToTop(hWnd)) {
    std::cout << "GetWindowPlacement Failed!\n";
  }
}

void printWindowCoordinates(HWND &hWnd, const std::string &windowTitle) {
  auto wpl = getWindowPlacement(hWnd);
  std::cout << hWnd << ": " << windowTitle << "{\n"
            << "\tleft up (" << wpl.rcNormalPosition.left << ", "
            << wpl.rcNormalPosition.top << "),\n\tright down ("
            << wpl.rcNormalPosition.right << "," << wpl.rcNormalPosition.bottom
            << ")\n}\n";
}

void ifVisible(HWND hWnd, const std::string &windowTitle) {
  // List visible windows with a non-empty title
  if (!IsWindowVisible(hWnd) || windowTitle.empty()) {
    return;
  }

  if (IsIconic(hWnd)) {
    std::cout << "--- DEBUG --- line:" << __LINE__
              << " | IsIconic(hWnd): " << windowTitle << '\n';
  }

  // if () {
  // 	std::cout << "--- DEBUG --- line:" << __LINE__ << " | !IsIconic(hWnd): "
  // << windowTitle << '\n';
  // }
  // todo: limit only to messenger window
  // if (windowTitle != "Messenger") {
  //   return;
  // }

  char class_name[80];
  GetClassName(hWnd, class_name, sizeof(class_name));
  const std::string class_name_str{class_name};
  // std::cout << "--- DEBUG --- line:" << __LINE__ << " | GetWindowLong(hWnd,
  // GWLP_USERDATA) class_name:" << class_name << '\n';

  if (GetWindowLong(hWnd, GWL_STYLE) &&
      class_name_str != "ApplicationFrameWindow") {
    std::cout << "--- DEBUG --- line:" << __LINE__
              << " | GetWindowLong(hWnd, GWLP_USERDATA) windowTitle: "
              << windowTitle << ", class_name: " << class_name_str << '\n';
  }
  // printWindowCoordinates(hWnd, windowTitle);
  // setWindowPos(hWnd);

  DWORD processID = 0;
  if (auto threadId = GetWindowThreadProcessId(hWnd, &processID); threadId) {
    std::cout << "--- DEBUG --- line:" << __LINE__
              << " | GetWindowThreadProcessId! processID: " << processID
              << ", threadId: " << threadId << " | ";
    PrintProcessNameAndID(processID);
  }
}

BOOL CALLBACK enumWindowCallback(HWND hWnd, LPARAM lparam) {
  int length = GetWindowTextLength(hWnd);
  char *buffer = new char[length + 1];
  GetWindowText(hWnd, buffer, length + 1);
  const std::string windowTitle(buffer);

  ifVisible(hWnd, windowTitle);

  return TRUE;
}

void doTheThingWithWindows() {
  std::cout << "Enmumerating windows..." << std::endl;
  EnumWindows(enumWindowCallback, NULL);
}

void setupExitToTray(QQmlApplicationEngine &engine) {
  QObject *root = 0;
  if (engine.rootObjects().size() > 0) {
    root = engine.rootObjects().at(0);

    QAction *minimizeAction = new QAction(QObject::tr("Mi&nimize"), root);
    root->connect(minimizeAction, SIGNAL(triggered()), root, SLOT(hide()));
    QAction *maximizeAction = new QAction(QObject::tr("Ma&ximize"), root);
    root->connect(maximizeAction, SIGNAL(triggered()), root,
                  SLOT(showMaximized()));
    QAction *restoreAction = new QAction(QObject::tr("&Restore"), root);
    root->connect(restoreAction, SIGNAL(triggered()), root, SLOT(showNormal()));
    QAction *quitAction = new QAction(QObject::tr("&Quit"), root);
    root->connect(quitAction, SIGNAL(triggered()), qApp, SLOT(quit()));

    QMenu *trayIconMenu = new QMenu();
    trayIconMenu->addAction(minimizeAction);
    trayIconMenu->addAction(maximizeAction);
    trayIconMenu->addAction(restoreAction);
    trayIconMenu->addSeparator();
    trayIconMenu->addAction(quitAction);

    QSystemTrayIcon *trayIcon = new QSystemTrayIcon(root);
    trayIcon->setContextMenu(trayIconMenu);
    trayIcon->setIcon(QIcon(
        "C:/Users/PrzemyslawPorebski/Projects/sentinel/gui/sentinelico64.ico"));
    trayIcon->show();
  }
}

int main(int argc, char *argv[]) {
  QApplication app(argc, argv);

  if (!QSystemTrayIcon::isSystemTrayAvailable()) {
    QMessageBox::critical(0, QObject::tr("Systray"),
                          QObject::tr("I couldn't detect any system tray "
                                      "on this system."));
    return 1;
  }
  // QApplication::setQuitOnLastWindowClosed(false);

  QQmlApplicationEngine engine;
  engine.load(
      QUrl::fromLocalFile(QFileInfo("qml/main.qml").absoluteFilePath()));

  setupExitToTray(engine);

  doTheThingWithWindows();

  return app.exec();
}
