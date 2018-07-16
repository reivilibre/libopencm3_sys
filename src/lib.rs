#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod raw_c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

