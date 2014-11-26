#![feature(phase)]
#![feature(globs)]

#[phase(plugin)]
extern crate gfx_macros;

extern crate current;
extern crate shader_version;
extern crate vecmath;
extern crate event;
extern crate input;
extern crate cam;
extern crate gfx;
extern crate device;
extern crate sdl2;
extern crate sdl2_window;
extern crate time;
extern crate image;
extern crate assimp;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

use assimp as ai;
use current::{ Set };
use image::GenericImage;
use sdl2_window::Sdl2Window;
use gfx::{ Device, DeviceHelper, ToSlice };
use event::{ Events, WindowSettings };
use event::window::{ CaptureCursor };

const MAX_BONES: uint = 60;
type BoneTranslation = [f32, ..4];
type BoneRotation = [f32, ..4];

struct TextureStore {
    textures: HashMap<String, gfx::TextureHandle>,
}

impl TextureStore {
    fn new(directory: &str,
           device: &mut gfx::GlDevice,
           ) -> TextureStore {

        let mut textures = HashMap::new();

        let dir = Path::new(directory);
        let stuff = io::fs::readdir(&dir).unwrap();
        for path in stuff.iter() {
            match path.extension_str() {
                None => continue,
                Some(ext) => if ext != "tga" {
                    continue;
                },
            }
            let mut img = image::open(path).unwrap();
            let (w, h) = img.dimensions();
            img = image::DynamicImage::ImageRgba8(img.to_rgba());
            assert!(img.color() == image::RGBA(8));

            let tinfo = gfx::tex::TextureInfo {
                width: w as u16,
                height: h as u16,
                depth: 1,
                levels: 1,
                kind: gfx::tex::Texture2D,
                format: gfx::tex::RGBA8,
            };

            let img_info = tinfo.to_image_info();
            let texture = device.create_texture(tinfo).unwrap();
            device.update_texture(
                    &texture,
                    &img_info,
                    img.raw_pixels().as_slice(),
                ).unwrap();

            match path.filename_str() {
                Some(fname) => {
                    println!("Loaded texture: {}", fname);
                    textures.insert(fname.into_string(), texture);
                },
                None => panic!("Couldn't create texture from image"),
            }
        }

        TextureStore {
            textures: textures
        }
    }
}

struct ModelComponent {
    pub batch: ModelBatch,
    pub shader_data: ShaderParam,
}

struct BoneStore {
    /// Translates a bone name into a bone id
    pub bone_map: HashMap<String, gfx::TextureHandle>,
    // pub bone_list: Vec<Bones>
    pub num_bones: uint;
}

struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub batches: Vec<ModelComponent>,
}

impl Model {
    fn from_file(fname: &str,
                 // device: &mut gfx::GlDevice,
                 graphics: &mut gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>,
                 program: &gfx::ProgramHandle,
                 state: &gfx::DrawState,
                 texture_store: &TextureStore,
                 ) -> Model {
        let mut importer = ai::Importer::new();
        importer.add_processing_steps(&[
                                       ai::Process::Triangulate,
                                       // ai::Process::GenNormals,
                                       ai::Process::GenSmoothNormals,
                                       ai::Process::JoinIdenticalVertices
                                       ]);

        let ai_scene = match importer.import_from_file(fname) {
            Some(scene) => scene,
            None => panic!("failed to import scene: {}", fname),
        };

        // calculate the space we need to allocate
        let mut num_vertices = 0;
        let mut num_indices = 0;
        for mesh in ai_scene.get_meshes().iter() {
            num_vertices += mesh.num_vertices;
            num_indices += mesh.num_faces * 3;
        }

        // prepare the data structures used to store the scene
        let mut vertices = Vec::with_capacity(num_vertices as uint);
        let mut indices = Vec::with_capacity(num_indices as uint);
        // stores the first index of each mesh
        let mut start_indices = Vec::with_capacity(ai_scene.num_meshes as uint + 1);
        let mut batches = Vec::with_capacity(ai_scene.num_meshes as uint);
        let mut materials = Vec::with_capacity(ai_scene.num_materials as uint);

        // find the textures used by this model from the list of materials
        for mat in ai_scene.get_materials().iter() {
            let texture_src = mat.get_texture(ai::material::TextureType::Diffuse,
                                              0
                                             );
            match texture_src {
                Some(s) => {
                    match texture_store.textures.get(&s) {
                        Some(t) => materials.push(t),
                        None => panic!("couldn't load texture: {}", s),
                    }
                }
                None => {
                    panic!("could read texture name from material: {}", texture_src);
                }
            }
        }


        // prepare the data for a format that can be loaded to the gpu
        {
            start_indices.push(0);

            for mesh in ai_scene.get_meshes().iter() {
                let vert_count  = vertices.len() as u32;

                let verts = mesh.get_vertices();
                let norms = mesh.get_normals();
                //TODO handle no texture coords
                let tex_coords = mesh.get_texture_coords()[0];


                for i in range(0u, verts.len()) {
                    vertices.push( Vertex {
                        a_position: verts[i].to_slice(),
                        a_normal: norms[i].to_slice(),
                        a_tex_coord: tex_coords[i].to_slice(),
                    });
                }

                for face in mesh.get_faces().iter() {
                    let face_indices = face.get_indices();
                    assert!(face_indices.len() == 3);
                    indices.push(face_indices[0] + vert_count);
                    indices.push(face_indices[1] + vert_count);
                    indices.push(face_indices[2] + vert_count);
                }

                start_indices.push(indices.len() as u32);

                // all the bones referenced by this mesh
                for bone in mesh.get_bones().iter() {

                }

            }
        }


        // create the vertex and index buffers
        // generate the batches used to draw the object
        {
            let vert_buf = graphics.device.create_mesh(vertices.as_slice());
            let ind_buf = graphics.device.create_buffer_static(indices.as_slice());

            let mut buf_slices = Vec::with_capacity(ai_scene.num_meshes as uint + 1);

            for ind in start_indices.windows(2) {
                buf_slices.push(gfx::Slice {
                    start: ind[0],
                    end: ind[1],
                    prim_type: gfx::TriangleList,
                    // prim_type: gfx::LineStrip,
                    kind: gfx::SliceKind::Index32(ind_buf, 0 as u32),
                });
            }

            for (slice, mesh) in buf_slices.iter()
                                 .zip(ai_scene.get_meshes().iter()) {
                let u_bone_translation: gfx::BufferHandle<BoneTranslation> =
                    graphics.device.create_buffer(MAX_BONES, gfx::BufferUsage::Dynamic);
                let u_bone_rotation: gfx::BufferHandle<BoneRotation> =
                    graphics.device.create_buffer(MAX_BONES, gfx::BufferUsage::Dynamic);
                let shader_data = ShaderParam {
                    u_model_view_proj: vecmath::mat4_id(),
                    t_color: (*materials[mesh.material_index as uint], None),
                    u_bone_translation: u_bone_translation.raw(),
                    u_bone_rotation: u_bone_rotation.raw(),
                };

                batches.push(ModelComponent {
                    batch: graphics.make_batch(program,
                                               &vert_buf,
                                               *slice,
                                               state).unwrap(),
                    shader_data: shader_data,
                });
            }
        }

        Model {
            vertices: vertices,
            indices: indices,
            batches: batches,
        }
    }

    fn draw(&mut self,
            graphics: &mut gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>,
            frame: &gfx::Frame,
            transform: [[f32, ..4], ..4],
            ) {
        for &mut component in self.batches.iter() {
            component.shader_data.u_model_view_proj = transform;
            graphics.draw(&component.batch, &component.shader_data, frame);
        }
    }
}

#[vertex_format]
struct Vertex {
    #[as_float]
    a_position: [f32, ..3],
    #[as_float]
    a_normal: [f32, ..3],
    #[as_float]
    a_tex_coord: [f32, ..3],
    // #[as_float]
    // a_weights: [f32, ..4],
    // #[as_float]
    // a_bone_ids: [u8, ..4],
    //TODO bones
}

#[shader_param(ModelBatch)]
struct ShaderParam {
    u_model_view_proj: [[f32, ..4], ..4],
    /// texture for the mesh
    t_color: gfx::shade::TextureParam,
    /// mesh rotations caused by bones
    u_bone_rotation: gfx::RawBufferHandle,
    /// mesh transformations caused by bones
    u_bone_translation: gfx::RawBufferHandle,
}

static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {//{{{
GLSL_150: b"
    #version 150 core
    in vec3 a_position;
    in vec3 a_normal;
    in vec3 a_tex_coord;
    // in vec4 a_weights;
    // in ivec4 a_bone_ids;

    out vec2 v_TexCoord;

    uniform mat4 u_model_view_proj;
    uniform u_bone_rotation {
        vec4[60] quat;
    };
    uniform u_bone_translation {
        vec4[60] offset;
    };

    void main() {
        v_TexCoord = vec2(a_tex_coord);
        gl_Position = u_model_view_proj * vec4(a_position, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;

    out vec4 o_Color;
    uniform sampler2D t_color;
    void main() {
        vec4 tex = texture(t_color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
};//}}}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
     std::rt::start(argc, argv, main)
}

fn main() {
    let (win_width, win_height) = (640, 480);
    let mut window = Sdl2Window::new(
        shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "model".to_string(),
            size: [win_width, win_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    window.set_mut(CaptureCursor(true));

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let frame = gfx::Frame::new(win_width as u16, win_height as u16);
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    ai::log::add_log_stream(ai::log::Stdout);

    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(
            gfx::tex::Bilinear,
            gfx::tex::Clamp
        )
    );

    let program = device.link_program(
            VERTEX_SRC.clone(),
            FRAGMENT_SRC.clone()
    ).unwrap();

    let texture_store = TextureStore::new("../assets/guard-md5",
                                          &mut device
                                         );

    let mut graphics = gfx::Graphics::new(device);
    let mut model = Model::from_file("../assets/guard-md5/guard.md5mesh",
                                     &mut graphics,
                                     &program,
                                     &state,
                                     &texture_store,
                                     );


    // Rotate the model 90 deg around the x-axis
    let model_view =
    [
        [ 1.0,  0.0,  0.0,  0.0],
        [ 0.0,  0.0, -1.0,  0.0],
        [ 0.0,  1.0,  0.0,  0.0],
        [ 0.0,  0.0,  0.0,  1.0],
    ];

    let projection = cam::CameraPerspective {
            fov: 90.0f32,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (win_width as f32) / (win_height as f32)
        }.projection();

    let mut first_person = cam::FirstPerson::new(
        [10.5f32, 0.5, 9.0],
        cam::FirstPersonSettings::keyboard_wasd()
    );
    first_person.velocity = 30.0f32;
    first_person.settings.speed_vertical = 30.0f32;

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::RenderEvent;

        first_person.event(&e);
        e.render(|args| {
            graphics.clear(
                gfx::ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                },
                gfx::COLOR | gfx::DEPTH,
                &frame
            );

            let u_model_view_proj = cam::model_view_projection(
                model_view,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );

            model.draw(&mut graphics,
                       &frame,
                       u_model_view_proj,
                       );

            graphics.end_frame();
        });
    }
}
