// SPDX-License-Identifier: GPL-2.0-or-later
mod single_instance;

// These modules preserve the ABI names and record layouts of PCManFM's GTK,
// libfm and X11 callbacks. GTK dispatches all application callbacks on its main thread.
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_assignments,
    dead_code,
    unused_unsafe,
    unused_parens,
    unused_variables,
    unused_imports,
    path_statements,
    static_mut_refs,
    non_fmt_panics,
    unpredictable_function_pointer_comparisons
)]
mod app;
pub use app::run;
