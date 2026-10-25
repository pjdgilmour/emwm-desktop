cmake_minimum_required(VERSION 3.16)

project(emwm-wallpaper-tool VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTKMM REQUIRED gtkmm-3.0)
pkg_check_modules(X11 REQUIRED IMPORTED_TARGET x11)
pkg_check_modules(XRANDR REQUIRED IMPORTED_TARGET xrandr)
pkg_check_modules(PIXBUF REQUIRED IMPORTED_TARGET gdk-pixbuf-2.0)

add_library(backdrops STATIC src/backdrops.cpp)
target_include_directories(backdrops PUBLIC src)
target_link_libraries(backdrops PUBLIC PkgConfig::X11)
target_compile_options(backdrops PRIVATE -Wall -Wextra -pedantic)

add_library(wallpaper STATIC src/wallpaper.cpp)
target_link_libraries(wallpaper PUBLIC backdrops PkgConfig::PIXBUF PkgConfig::XRANDR)
target_compile_options(wallpaper PRIVATE -Wall -Wextra -pedantic)

add_executable(emwm-wallpaper-tool
    src/main.cpp
)

target_include_directories(emwm-wallpaper-tool PRIVATE ${GTKMM_INCLUDE_DIRS})
target_link_directories(emwm-wallpaper-tool PRIVATE ${GTKMM_LIBRARY_DIRS})
target_link_libraries(emwm-wallpaper-tool PRIVATE wallpaper ${GTKMM_LIBRARIES})
target_compile_options(emwm-wallpaper-tool PRIVATE ${GTKMM_CFLAGS_OTHER} -Wall -Wextra -pedantic)

include(CTest)
if(BUILD_TESTING)
    add_executable(test-backdrops tests/backdrops.cpp)
    target_link_libraries(test-backdrops PRIVATE backdrops)
    add_test(NAME backdrop-persistence COMMAND test-backdrops)
    add_executable(test-wallpaper tests/wallpaper.cpp)
    target_link_libraries(test-wallpaper PRIVATE wallpaper)
    add_test(NAME wallpaper-placement COMMAND test-wallpaper)
    add_executable(test-wallpaper-ui tests/ui.cpp)
    target_include_directories(test-wallpaper-ui PRIVATE ${GTKMM_INCLUDE_DIRS})
    target_link_directories(test-wallpaper-ui PRIVATE ${GTKMM_LIBRARY_DIRS})
    target_link_libraries(test-wallpaper-ui PRIVATE wallpaper ${GTKMM_LIBRARIES})
    target_compile_options(test-wallpaper-ui PRIVATE ${GTKMM_CFLAGS_OTHER} -Wall -Wextra -pedantic)
    # Optional integration test: requires Xvfb, xrdb, tellmwm and an EMWM binary.
    add_custom_target(test-x11
        COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/tests/run-x11.sh
            $<TARGET_FILE:test-backdrops> $<TARGET_FILE:emwm-wallpaper-tool> $<TARGET_FILE:test-wallpaper> $<TARGET_FILE:test-wallpaper-ui>
        DEPENDS test-backdrops test-wallpaper test-wallpaper-ui emwm-wallpaper-tool
        USES_TERMINAL)
endif()

install(TARGETS emwm-wallpaper-tool RUNTIME DESTINATION bin)
