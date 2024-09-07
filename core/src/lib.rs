extern crate self as astrix_core;

pub mod assert;
pub mod console;
pub mod astrixd_env;
pub mod log;
pub mod panic;
pub mod time;

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub mod core;
        pub mod service;
        pub mod signals;
        pub mod task;
    }
}
