//! The crate's integration tests, one module per file, built as one test binary: each
//! binary links the whole dependency graph, so one instead of one per file builds much
//! faster (#2204). Add new test files here and list them below.

#[path = "../common/mod.rs"]
mod common;

mod camera_raw;
mod decode_warnings;
mod deep_exr;
mod detect_caps;
mod exr_cryptomatte;
mod exr_multipart;
mod fidelity;
mod heif;
mod limits;
mod malformed;
mod metadata;
mod orientation;
mod png_jpeg;
mod resolution;
mod roundtrip;
mod tiff_bench;
mod tiff_pages;
mod tiff_photoshop;
mod tiff_regressions;
mod webp_lossy;
