#![allow(
    function_casts_as_integer,
    static_mut_refs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate cstr;

rust_i18n::i18n!("assets/locales", fallback = "en");

#[macro_use]
pub mod core;
pub mod il2cpp;

/** Android **/
#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
use android::{game_impl, gui_impl, hachimi_impl, interceptor_impl, log_impl, symbols_impl};

/** Windows **/
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows::{game_impl, gui_impl, hachimi_impl, interceptor_impl, log_impl, symbols_impl};
