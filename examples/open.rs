extern crate assimp;

use assimp as ai;

fn main() {
    // Log to stdout and a file `log.txt`
    ai::log::add_log_stream(ai::log::LogStreamStdout);
    ai::log::add_log_stream(ai::log::LogStreamFile("log.txt"));
    ai::log::enable_verbose_logging(true);

    let importer = ai::Importer::new();

    // The file to import
    let scene = importer.import("examples/assets/cube.dae").unwrap();

    // Print all the vertices in all the meshes
    for mesh in scene.get_meshes().iter() {
        println!("Mesh.name: {}", mesh.name);
        for vert in mesh.get_vertices().iter() {
            println!("{}", vert);
        }
    }
}
