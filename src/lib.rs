#![feature(doc_auto_cfg)]

//!
//! Logic is a modern *structural logic* library based on equivalence graphs.
//!
//! It allows defining a **(term-based) language** as an abstract syntax tree (AST) associated with **rewriting rules** expressing *equivalence relations*.
//!
//! ![Example](https://logic-lang.github.io/assets/img/logic-example.png)
//! > *Transformation of an AST using predefined rewriting rules*.
//!
//! There are three stages in a logic setup, which are analogies to intermediate representations (IR) in translation systems.
//! Hierarchically, they depend on each other:
//!
//! 1. [`logic_cir`] is the top-level *rewriting* algorithm.
//! 2. [`logic_mir`] is used for *pattern matching*.
//! 3. [`logic_hir`] is used for *equivalence* representations.
//!
//! #### HIR (high-level IR)
//!
//! Expressions are lowered to an index-based representation that only exists in an equivalence graph.
//! At this stage, it is possible to *test for equivalence* and *simplify terms* based on a cost function.
//!
//! #### MIR (mid-level IR)
//!
//! In order to detect rule patterns, expressions can be compiled into decision trees.
//! These allows for *matching* and subtrees (terms) substitution.
//!
//! #### CIR (ctx-level IR)
//!
//! Finally, matches can be applied automatically to the equivalence graph using *equality saturation*.
//! This is the basis of the rewriting algorithm.
//!
//! ### Extension ideas
//!
//! Minimizing rule sets and determining if a rewriting system is terminating through graph algorithms might be investigated in the future.
//!
//! # A tour of the logic library
//!
//! The crate documentation also aims to provide important informations about the state of the project and some (planned) additional features.
//!
//! ### State: experiment
//!
//! * The public API is constantly evolving and substantial changes are expected prior to stable releases (x.0.0).
//! * Feel free to suggest problems that could help to improve the library and provide realistic use cases.
//! * Help is welcomed.
//!
//! ### Features
//!
//! * `logic_lang` ([`lang`]) — traits for parsing, interpretation (and later, bytecode compilation) of expression structures.
//!
//! ### References
//!
//! * *Graph Rewrite Systems for Program Optimization.* [(doi)](https://doi.org/10.1145/363911.363914)
//! * *Simplify: A Theorem Prover for Program Checking.* [(doi)](https://doi.org/10.1145/1066100.1066102)
//! * rust-lang docs.
//!

pub use ty::Type;
pub mod logic_cir;
pub mod logic_hir;
pub mod logic_mir;
#[macro_use]
mod ty;

#[cfg(feature = "logic_lang")]
pub mod lang;
