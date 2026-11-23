cmake_minimum_required(VERSION 3.16)
project(emwm-compositor VERSION 0.2.7 LANGUAGES NONE)
find_program(CARGO cargo REQUIRED)
set(crate "${CMAKE_CURRENT_SOURCE_DIR}/../../rust/compositor")
set(target_dir "${CMAKE_CURRENT_BINARY_DIR}/cargo")
set(cargo_args --locked --manifest-path "${crate}/Cargo.toml" --target-dir "${target_dir}" --release)
# Cargo tracks Rust inputs and skips compilation when its build is current.
add_custom_target(emwm-compositor ALL
    COMMAND "${CARGO}" build ${cargo_args}
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${target_dir}/release/emwm-compositor" "${CMAKE_CURRENT_BINARY_DIR}/emwm-compositor"
    VERBATIM)
include(CTest)
if(BUILD_TESTING)
    add_test(NAME compositor-tests COMMAND "${CARGO}" test ${cargo_args})
    configure_file(tests/test-render-effects.in test-render-effects @ONLY)
endif()
install(PROGRAMS "${CMAKE_CURRENT_BINARY_DIR}/emwm-compositor" DESTINATION bin)
