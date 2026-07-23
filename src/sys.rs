#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(
    clippy::approx_constant,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::useless_transmute
)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
