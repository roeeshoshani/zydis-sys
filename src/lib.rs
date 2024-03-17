//! low level rust bindings for the zydis library for encoding/decoding and messing with x86 instructions.
//! for higher level bindings, check out the `rydis` crate.

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/zydis.rs"));
