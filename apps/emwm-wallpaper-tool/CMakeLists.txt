cmake_minimum_required(VERSION 3.16)

project(emwm-wallpaper-tool VERSION 1.0.0 LANGUAGES NONE)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_program(CARGO cargo)
if(NOT CARGO)
    message(FATAL_ERROR "Cargo is required to build emwm-wallpaper-tool")
endif()
set(CRATE "${CMAKE_CURRENT_SOURCE_DIR}/../../rust/wallpaper")
set(CARGO_OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/cargo")
if(DEFINED ENV{CARGO_TARGET_DIR})
    get_filename_component(CARGO_OUTPUT "$ENV{CARGO_TARGET_DIR}" ABSOLUTE)
endif()
set(WALLPAPER "${CMAKE_CURRENT_BINARY_DIR}/emwm-wallpaper-tool")
file(GLOB RUST_SOURCES CONFIGURE_DEPENDS "${CRATE}/src/*.rs")
add_custom_command(
    OUTPUT "${WALLPAPER}"
    COMMAND "${CARGO}" build --release --locked --manifest-path "${CRATE}/Cargo.toml" --target-dir "${CARGO_OUTPUT}"
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${CARGO_OUTPUT}/release/emwm-wallpaper-tool" "${WALLPAPER}"
    DEPENDS ${RUST_SOURCES} "${CRATE}/Cargo.toml" "${CRATE}/Cargo.lock"
    VERBATIM)
add_custom_target(wallpaper-rust ALL DEPENDS "${WALLPAPER}")

include(CTest)
if(BUILD_TESTING)
    enable_language(CXX)
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(GTKMM REQUIRED IMPORTED_TARGET gtkmm-3.0)
    pkg_check_modules(X11 REQUIRED IMPORTED_TARGET x11)
    pkg_check_modules(XRANDR REQUIRED IMPORTED_TARGET xrandr)
    pkg_check_modules(PIXBUF REQUIRED IMPORTED_TARGET gdk-pixbuf-2.0)

    # The C++ libraries provide independent fixtures for the original file
    # formats, monitor coordinates and expected pixel colors.
    add_library(backdrops STATIC src/backdrops.cpp)
    target_include_directories(backdrops PUBLIC src)
    target_link_libraries(backdrops PUBLIC PkgConfig::X11)
    add_library(wallpaper STATIC src/wallpaper.cpp)
    target_link_libraries(wallpaper PUBLIC backdrops PkgConfig::PIXBUF PkgConfig::XRANDR)
    add_test(NAME wallpaper-rust COMMAND "${CARGO}" test --release --locked
             --manifest-path "${CRATE}/Cargo.toml" --target-dir "${CARGO_OUTPUT}")
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
    find_program(RUST_ANYTHING rust-anything HINTS "$ENV{HOME}/.cargo/bin")
    if(RUST_ANYTHING)
        add_executable(wallpaper-reference EXCLUDE_FROM_ALL src/main.cpp)
        target_link_libraries(wallpaper-reference PRIVATE wallpaper PkgConfig::GTKMM)
        add_custom_target(test-cli-parity
            COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/tests/compare-cli.sh
                $<TARGET_FILE:wallpaper-reference> "${WALLPAPER}"
                ${CMAKE_CURRENT_SOURCE_DIR}/tests/cli-cases.jsonl $<TARGET_FILE:test-backdrops>
                "${RUST_ANYTHING}"
            DEPENDS wallpaper-reference wallpaper-rust test-backdrops VERBATIM)
    endif()
    # Optional integration test: requires Xvfb, xrdb, tellmwm and an EMWM binary.
    add_custom_target(test-x11
        COMMAND sh ${CMAKE_CURRENT_SOURCE_DIR}/tests/run-x11.sh
            $<TARGET_FILE:test-backdrops> "${WALLPAPER}" $<TARGET_FILE:test-wallpaper> $<TARGET_FILE:test-wallpaper-ui>
            "${CARGO}" "${CRATE}/Cargo.toml" "${CARGO_OUTPUT}"
        DEPENDS test-backdrops test-wallpaper test-wallpaper-ui wallpaper-rust
        USES_TERMINAL)
endif()

install(PROGRAMS "${WALLPAPER}" DESTINATION bin)
