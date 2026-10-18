//! `type` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `type` capability surface entry.
pub const TYPE_HNL: &str = include_str!("index.hnl");

/// `type.hnl` — the interface decl variant.
pub const TYPE_INTERFACE_HNL: &str = include_str!("type.hnl");

/// `dependent.hnl` — indexed families and dependent structure (ADR-0120).
pub const DEPENDENT_HNL: &str = include_str!("dependent.hnl");
