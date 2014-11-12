//! A binding for Open Asset Import Library (assimp)

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

pub mod animation;
pub mod camera;
pub mod import;
pub mod light;
pub mod material;
pub mod mesh;
pub mod scene;
pub mod texture;
pub mod types;
pub mod version;

#[cfg(test)]
mod test {
    use version;

    #[test]
    fn test_version() {
        // Hello world test
        let (major, minor) = version::get_version();
        println!("{}, {}", major, minor);
    }
}
