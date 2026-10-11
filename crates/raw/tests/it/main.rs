//! The crate's integration tests, one module per file, built as one test binary: each
//! binary links the whole dependency graph, so one instead of one per file builds much
//! faster (#2204). Add new test files here and list them below.

mod dng;
mod hostile;
mod raf;
mod raw_corpus;
mod vendor_codecs;
mod vendors;
