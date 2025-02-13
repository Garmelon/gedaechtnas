// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// This warning does not apply here - the lib uses the dependencies.
#![allow(unused_crate_dependencies)]

fn main() {
    gdn_app_lib::run()
}
