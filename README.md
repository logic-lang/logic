![Example](https://logic-lang.github.io/assets/img/logic-logo.png)

[![License](https://img.shields.io/github/license/logic-lang/logic?color=informational&style=flat-square)](https://github.com/logic-lang/logic/blob/master/LICENSE)
[![Crate](https://img.shields.io/crates/v/logic-lang?style=flat-square)](https://crates.io/crates/logic-lang)
[![Docs](https://img.shields.io/docsrs/logic-lang?style=flat-square&logo=docsdotrs)](https://docs.rs/logic-lang)
[![Build](https://img.shields.io/github/actions/workflow/status/logic-lang/logic/ci.yml?branch=main&style=flat-square&logo=githubactions)](https://github.com/logic-lang/logic/actions)

---

Logic is a modern *structural logic* library based on equivalence graphs.

Logic can prove equivalences and optimize expressions according to some criterion.
It allows defining a **(term-based) language** as an abstract syntax tree (AST) associated with the **rewriting rules** expressing its *equivalence relations*. 
Applying rules is automatically performed by a three-stages process based on different "intermediate representations (IRs)".

---

> **Note**: this is an exploratory project.

## Getting started

An extensive tutorial/book is planned. Until then, you can still read the [documentation](https://docs.rs/logic-lang) for the latest version or browse 
the sources.

It is recommanded to use *nightly* Rust but Logic should build just fine on *stable* (MSRV 1.69).
The standard cargo `fmt`, `clippy`, `test` and `bench` workflow is available.

#### Try the [web demo](https://logic-lang.github.io/) running on WebAssembly.

* [Paper](#)
* [Tutorial](#tutorial)
* [Apps](#apps)
* [Citing](#citing)
