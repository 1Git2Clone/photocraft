//! The crate's integration tests, one module per file, built as one test binary: each
//! binary links the whole dependency graph, so one instead of one per file builds much
//! faster (#2204). Add new test files here and list them below.

mod agent_tasks;
mod inactive_save;
mod layered_save_dirty;
mod mcp;
mod mcp_wire;
mod openraster;
