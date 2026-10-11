//! The crate's integration tests, one module per file, built as one test binary: each
//! binary links the whole dependency graph, so one instead of one per file builds much
//! faster (#2204). Add new test files here and list them below.

#[path = "../common/mod.rs"]
mod common;

mod analysis;
mod autosave;
mod corruption;
mod layer_labels;
mod roundtrip;
mod smart_contents;
