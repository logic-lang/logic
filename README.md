<div align="center">

# logic 

![Example](https://logic-lang.github.io/assets/img/logic-example.png)

**structural logic based on equivalence graphs**

[![License](https://img.shields.io/github/license/logic-lang/logic?color=informational&style=flat-square)](https://github.com/logic-lang/logic/blob/master/LICENSE)
[![Crate](https://img.shields.io/crates/v/logic-lang?style=flat-square)](https://crates.io/crates/logic-lang)
[![Docs](https://img.shields.io/docsrs/logic-lang?style=flat-square&logo=docsdotrs)](https://docs.rs/logic-lang)
[![Build](https://img.shields.io/github/actions/workflow/status/logic-lang/logic/ci.yml?branch=main&style=flat-square&logo=githubactions)](https://github.com/logic-lang/logic/actions)

</div>

---

Logic is a modern *structural logic* library based on equivalence graphs.
It allows defining a **(term-based) language** as a generic abstract syntax tree (AST) associated with **rewriting rules** expressing *equivalence relations*.
Lowering expressions to index-based *hir* then allows for **matching and substituting** the current representation using equality saturation.
**Minimizing** rule sets or determining if a rewriting system is **terminating** through graph algorithms might be investigated in the future.

---

> **Note**: this is an exploratory project.

## Current state

* The public API is constantly evolving and substantial changes are expected prior to stable releases (x.0.0).
* Feel free to suggest problems that could help to improve the library and provide realistic use cases.
* Help is welcomed.

## Getting started

An extensive tutorial is currently under construction, but you can still read the [documentation](https://docs.rs/logic-lang) for the latest version or browse the sources.

It is recommanded to use *nightly* Rust but Logic should build just fine on *stable* (MSRV 1.69).
The standard cargo `fmt`, `clippy`, `test` and `bench` workflow is available.

#### Try the [web demo](https://logic-lang.github.io/) running on WebAssembly.

* [Website](#)
* [Blog](#blog)
* [Tutorial](#usage)
* [Citing](#citing)
