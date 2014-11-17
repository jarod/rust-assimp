rust-assimp [![Build Status](https://travis-ci.org/juxiliary/rust-assimp.svg?branch=master)](https://travis-ci.org/juxiliary/rust-assimp)
===========

[Documentation](http://www.rust-ci.org/juxiliary/rust-assimp/doc/assimp/)

## WARNING
This code is currently experimental.

## Building

## Examles

### Simple import example
This example sets up logging, loads a model and prints all its vertices to
stdout.

```rust
extern crate assimp;

use assimp as ai;

fn main() {
    // Log to stdout and a file `log.txt`
    ai::log::add_log_stream(ai::log::Stdout);
    ai::log::add_log_stream(ai::log::File("log.txt"));
    ai::log::enable_verbose_logging(true);

    let importer = ai::Importer::new();

    // The file to import
    let scene = importer.import("examples/assets/cube.dae").unwrap();

    // Print all the vertices in all the meshes
    for mesh in scene.get_meshes().iter() {
        for vert in mesh.get_vertices().iter() {
            println!("{}", vert);
        }
    }
}
```
