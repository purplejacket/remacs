//! Functions to deal with files
use std::path;

use remacs_macros::lisp_fn;

use lisp::defsubr;
use multibyte::LispStringRef;

/// Return non-nil if NAME ends with a directory separator character.
#[lisp_fn]
pub fn directory_name_p(name: LispStringRef) -> bool {
    if name.len_bytes() == 0 {
        return false;
    }

    let b = name.byte_at(name.len_bytes() - 1);
    b as char == path::MAIN_SEPARATOR
}

include!(concat!(env!("OUT_DIR"), "/fileio_exports.rs"));
