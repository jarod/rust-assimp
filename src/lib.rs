//! A binding for Open Asset Import Library (assimp)

#![crate_name = "assimp"]
#![comment = "assimp binding for rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![doc(html_root_url = "http://www.rust-ci.org/juxiliary/rust-assimp/doc/assimp/")]

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(missing_docs)]
#![deny(unused_results)]
#![warn(unused_imports)]
#![deny(unused_typecasts)]

#![feature(globs)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]

#![experimental]

extern crate libc;

pub use types::{Vector2D, Vector3D, Color3D, Color4D, Matrix3x3, Matrix4x4,
                Quaternion, Plane, Ray, AiString};
pub use scene::{Scene};

pub use properties::*;
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
#[allow(missing_docs)] // don't need to document all vector elements
pub mod types;
pub mod importer;
pub mod log;

mod properties;
mod postprocess;
mod util;
mod ffi;
mod fileio;
