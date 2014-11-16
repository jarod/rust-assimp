//! A binding for Open Asset Import Library (assimp)

#![crate_name = "assimp"]
#![comment = "assimp binding for rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![doc(html_root_url = "http://www.rust-ci.org/juxiliary/rust-assimp/doc/assimp/")]

#![feature(globs)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]
#![allow(dead_code)]

extern crate libc;

pub use types::{Vector2D, Vector3D, Color3D, Color4D, Matrix3x3, Matrix4x4,
                Quaternion, Plane, Ray, AiString};
pub use scene::{Scene};

pub use config::*;
pub use postprocess::*;

pub use importer::Importer;

pub mod animation;
pub mod camera;
pub mod info;
pub mod light;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod texture;
pub mod types;
pub mod importer;
pub mod log;

mod config;
mod postprocess;
mod util;
mod ffi;
mod fileio;
