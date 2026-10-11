//! The crate's integration tests, one module per file, built as one test binary: each
//! binary links the whole dependency graph, so one instead of one per file builds much
//! faster (#2204). Add new test files here and list them below.

#[path = "../common/mod.rs"]
mod common;

#[path = "../support/current_affinity.rs"]
mod fixture;

mod adjust_blend_roundtrip;
mod adjust_warnings;
mod affinity;
mod affinity_artboards;
mod affinity_corpus;
mod annotations;
mod artboard_guide_order;
mod banded;
mod cancel;
mod channels;
mod color;
mod composite;
mod comps_artboards;
mod corpus;
mod corpus_regressions;
mod effects_multi;
mod exif_resolution;
mod flat;
mod heif;
mod heif_unsupported;
mod krita_corpus;
mod lab16;
mod large_psb;
mod layer_labels;
mod modes;
mod orientation;
mod pattern_fill_psd;
mod pdn_corpus;
mod png_text_metadata;
mod psd32_export;
mod psd_block_alignment;
mod psd_block_layout;
mod psd_empty_channels;
mod psd_kerning;
mod psd_structure;
mod psd_type_bounds;
mod psd_vertical_type;
mod raw;
mod resolution_axes;
mod roundtrip;
mod smart_contents_identity;
mod smart_corpus;
mod svg;
mod text_corpus;
mod text_styles;
mod tiff_corpus;
mod tiff_layers;
mod tiff_pages;
mod vector;
mod xmp_export;
