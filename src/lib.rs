#![feature(doc_auto_cfg)]

//!
//! Logic is a modern *structural logic* library based on equivalence graphs.
//!
//! It allows defining a **(term-based) language** as a generic abstract
//! syntax tree (AST) associated with **rewriting rules** expressing *equivalence relations*.
//! Lowering expressions to index-based *hir* then allows for **matching and substituting**
//! the current representation using equality saturation.
//! **Minimizing** rule sets or determining if a rewriting system is **terminating** through
//! graph algorithms might be investigated in the future.
//!
//! ![Example](https://logic-lang.github.io/assets/img/logic-example.png)
//! > *Transformation of an AST using predefined rewriting rules*.
//!
//! ## Current state
//!
//! * The public API is constantly evolving and substantial changes are expected prior to stable releases (x.0.0).
//! * Feel free to suggest problems that could help to improve the library and provide realistic use cases.
//! * Help is welcomed.
//!
//! ### Features
//!
//! * `logic_lang` ([`lang`]) — traits for parsing, interpretation (and later, bytecode compilation) of expression structures.
//!
//! ### Using
//!
//! It is recommanded to use *nightly* Rust but Logic should build just fine on *stable* (MSRV 1.69).
//! The standard cargo `fmt`, `clippy`, `test` and `bench` workflow is available.
//!
//! ### References
//!
//! * *Graph Rewrite Systems for Program Optimization.* [(doi)](https://doi.org/10.1145/363911.363914)
//! * *Simplify: A Theorem Prover for Program Checking.* [(doi)](https://doi.org/10.1145/1066100.1066102)
//! * rust-lang docs.
//!

pub mod logic_hir;
pub mod logic_mir;

#[cfg(feature = "logic_lang")]
pub mod lang;

pub mod rule;
pub mod ty;
