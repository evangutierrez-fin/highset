//! Domain types and IDs, the task status machine, events, flow types, the `ContextBundle` IR, the
//! config model and `paths`. No async and no IO except path resolution. This crate is a contract:
//! it freezes at milestone M1.
//!
//! See this crate's `README.md` for its lane and allowed internal dependencies.
#![forbid(unsafe_code)]
