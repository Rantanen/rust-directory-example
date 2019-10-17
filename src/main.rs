//! This file defines the primary crate.
//!
//! The compiler starts from this file and compiles everything referenced
//! through 'mod' statements.

// Include 'another_file.rs' as a module.
mod another_file;

// Include 'directory/mod.rs' as a module.
mod directory;

fn main() {
    println!("Hello, world!");

    // Here we can use the function from another_file with its
    // full name:
    crate::another_file::function();

    // Since this file is the crate root, we can also use it with
    // a relative name:
    another_file::function();

    // Finally we can bring the function alone into scope:
    use crate::another_file::function;
    function();

    directory::directory_function();
}
