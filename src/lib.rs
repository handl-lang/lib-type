//! `type` / `typeof` — Handl stdlib port.
//!
//! `index.hnl`/`type.hnl`/`dependent.hnl` are the `type` module surface;
//! `ops.rs` holds the native `typeof.*` `Op` impls and the
//! `type_op_registry()`/`TYPE_OPS` exports (moved verbatim from the
//! retired `kernel/crates/type` floor-lib crate — same package name).

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub mod ops;

pub use ops::{TYPE_OPS, type_op_registry};

/// `index.hnl` — the `type` capability surface entry.
pub const TYPE_HNL: &str = include_str!("index.hnl");

/// `type.hnl` — the interface decl variant.
pub const TYPE_INTERFACE_HNL: &str = include_str!("type.hnl");

/// `dependent.hnl` — indexed families and dependent structure (ADR-0120).
pub const DEPENDENT_HNL: &str = include_str!("dependent.hnl");
