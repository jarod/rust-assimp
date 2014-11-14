//! A binding for Open Asset Import Library (assimp)

#![feature(struct_variant)]
#![feature(unsafe_destructor)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

pub mod animation;
pub mod camera;
pub mod info;
pub mod light;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod texture;
pub mod types;

mod util;
mod import;
