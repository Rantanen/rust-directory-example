//! This is a file that is referenced from the main.rs directly.
//!
//! This makes the 'full path' of this file to be:
//! - `crate::another_file`

/// This is a function _exposed_ by this module.
///
/// Note the 'pub' keyword.
pub fn function() {
    println!("Called function");

    // We can call the internal function from here.
    internal_function();
}


/// This is a function limited to this module.
///
/// It has not been marked as 'pub'.
fn internal_function() {
    println!("Internal function");
}
