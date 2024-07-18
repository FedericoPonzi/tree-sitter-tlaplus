//! This crate provides Tlaplus language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(&tree_sitter_tlaplus::language()).expect("Error loading Tlaplus grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

#[cfg(feature = "native")]
use tree_sitter::Language;
#[cfg(feature = "wasm")]
use tree_sitter_c2rust::Language;

#[cfg(all(feature = "native", feature = "wasm"))]
compile_error!("feature \"native\" and feature \"wasm\" cannot be enabled at the same time");

extern "C" {
    fn tree_sitter_tlaplus() -> Language;
}

#[no_mangle]
pub extern "C" fn __assert_fail(
    assertion: *const u8,
    file: *const u8,
    line: u32,
    function: *const u8,
) {
    // Implement your assertion failure logic here
    // For example, print the assertion failure information
    unsafe {
        let assertion_str = std::ffi::CStr::from_ptr(assertion as *const i8);
        let file_str = std::ffi::CStr::from_ptr(file as *const i8);
        let function_str = std::ffi::CStr::from_ptr(function as *const i8);
        println!(
            "Assertion failed: {}, file: {}, line: {}, function: {}",
            assertion_str.to_str().unwrap(),
            file_str.to_str().unwrap(),
            line,
            function_str.to_str().unwrap()
        );
    }
}

#[no_mangle]
pub extern "C" fn iswspace(wc: u32) -> i32 {
    // Implement your iswspace logic here
    if wc == ' ' as u32 || wc == '\t' as u32 || wc == '\n' as u32 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswdigit(wc: u32) -> i32 {
    // Implement your iswdigit logic here
    if (wc >= '0' as u32) && (wc <= '9' as u32) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswalnum(wc: u32) -> i32 {
    // Implement your iswalnum logic here
    if ((wc >= '0' as u32) && (wc <= '9' as u32))
        || ((wc >= 'a' as u32) && (wc <= 'z' as u32))
        || ((wc >= 'A' as u32) && (wc <= 'Z' as u32))
    {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<usize>()).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    let layout = std::alloc::Layout::from_size_align(0, std::mem::align_of::<usize>()).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) }
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    let old_layout = std::alloc::Layout::from_size_align(0, std::mem::align_of::<usize>()).unwrap();
    let new_layout =
        std::alloc::Layout::from_size_align(size, std::mem::align_of::<usize>()).unwrap();
    unsafe { std::alloc::realloc(ptr, old_layout, new_layout.size()) }
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_tlaplus() }
}

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

// Uncomment these to include any queries that this grammar contains

pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
// pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[cfg(feature = "native")]
    use tree_sitter::Parser;
    #[cfg(feature = "wasm")]
    use tree_sitter_c2rust::Parser;

    #[test]
    fn test_can_load_grammar() {
        let mut parser = Parser::new();
        parser
            .set_language(&super::language())
            .expect("Error loading Tlaplus grammar");
    }
}
