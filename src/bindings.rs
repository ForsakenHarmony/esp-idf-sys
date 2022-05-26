#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::c_types;

// include!(env!("EMBUILD_GENERATED_BINDINGS_FILE"));
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
