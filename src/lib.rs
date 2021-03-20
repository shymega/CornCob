#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

pub(crate) mod instruction;
pub(crate) mod opcode;
pub mod vm;
