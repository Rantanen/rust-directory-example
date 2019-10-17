//! This is a module defined as a directory.
//!
//! The 'mod.rs' is a special file name that gets imported
//! when the directory is specified as a module.

// Let's expose a module that is contained in the directory:
mod directory_module;

pub fn directory_function() {

    // We can refer to the funtion from directory_module with
    // its full name:
    crate::directory::directory_module::dir_fn();

    // Since this file is the `directory` module, we can also
    // use a relative path:
    directory_module::dir_fn();

    // We could also import it into scope in this specific
    // function:
    use directory_module::dir_fn;
    dir_fn();
}
