#![feature(doc_auto_cfg)]

//!
//! Logic and API reference documentation.
//!

pub use ty::Type;
pub mod logic_cir;
pub mod logic_hir;
pub mod logic_mir;
#[macro_use]
mod ty;

#[cfg(feature = "logic_lang")]
pub mod lang;
