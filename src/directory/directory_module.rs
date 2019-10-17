//! This file specifies a `directory_module` module that
//! is located _within_ the `directory` module. 
//!
//! This makes the full path of this module:
//! - `crate::directory::directory_module`

pub fn dir_fn() {
    println!("Called crate::directory::directory_module::dir_fn");
}
