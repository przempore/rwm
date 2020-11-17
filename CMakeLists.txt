cmake_minimum_required(VERSION 3.16)

project(rwm LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

# don't show console window
if (WIN32)
    set(WIN32_ON_OFF_SWITCH "WIN32")
else ()
    set(WIN32_ON_OFF_SWITCH "")
endif ()

find_package(Qt5 COMPONENTS qml Core Widgets quick REQUIRED)

set(SOURCE winapi_connection/window_control.cpp)

add_executable(${PROJECT_NAME} main.cpp ${SOURCE})

target_link_libraries(${PROJECT_NAME} Qt5::Core Qt5::Widgets Qt5::Qml Qt5::Quick)

