//! RawScene function returned by c api functions.
use libc::{c_uint, c_void};

use scene::Node;
use animation::Animation;
use camera::Camera;
use light::Light;
use material::Material;
use mesh::Mesh;
use texture::Texture;
use types::{Matrix4x4, AiString, MemoryInfo};
use util::{ptr_ptr_to_slice, ptr_to_slice};
use postprocess::PostProcessSteps;

/// Objects of this class are generally maintained and owned by Assimp, not
/// by the caller. You shouldn't want to instance it, nor should you ever try to
/// delete a given scene on your own.
#[repr(C)]
pub struct RawScene {
    /// Any combination of the AI_SCENE_FLAGS_XXX flags.
    ///
    /// By default this value is 0, no flags are set. Most applications will
    /// want to reject all scenes with the AI_SCENE_FLAGS_INCOMPLETE bit set.
    pub flags: c_uint,

    /// The root node of the hierarchy.
    ///
    /// There will always be at least the root node if the import
    /// was successful (and no special flags have been set).
    /// Presence of further nodes depends on the format and content
    /// of the imported file.
    pub root_node: *mut Node,

    /// The number of meshes in the scene.
    pub num_meshes: c_uint,

    /// The array of meshes.
    ///
    /// Use the indices given in the aiNode structure to access
    /// this array. The array is mNumMeshes in size. If the
    /// AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
    /// be at least ONE material.
    pub meshes: *mut*mut Mesh,

    /// The number of materials in the scene.
    pub num_materials: c_uint,

    /// The array of materials.
    ///
    /// Use the index given in each aiMesh structure to access this
    /// array. The array is mNumMaterials in size. If the
    /// AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
    /// be at least ONE material.
    pub materials: *mut*mut Material,

    /// The number of animations in the scene.
    pub num_animations: c_uint,

    /// The array of animations.
    ///
    /// All animations imported from the given file are listed here.
    /// The array is mNumAnimations in size.
    pub animations: *mut*mut Animation,

    /// The number of textures embedded into the file
    pub num_textures: c_uint,

    /// The array of embedded textures.
    ///
    /// Not many file formats embed their textures into the file.
    /// An example is Quake's MDL format (which is also used by
    /// some GameStudio versions)
    pub textures: *mut*mut Texture,

    /// The number of light sources in the scene. Light sources
    /// are fully optional, in most cases this attribute will be 0
    pub num_lights: c_uint,

    /// The array of light sources.
    ///
    /// All light sources imported from the given file are listed here.  Light
    /// sources are fully optional, in most cases this array will contain 0.
    pub lights: *mut*mut Light,

    /// The number of cameras in the scene. Cameras
    /// are fully optional, in most cases this attribute will be 0
    pub num_cameras: c_uint,

    /// The array of cameras.
    ///
    /// All cameras imported from the given file are listed here.
    /// The array is mNumCameras in size. The first camera in the
    /// array (if existing) is the default camera view into
    /// the scene.
    pub cameras: *mut*mut Camera,

    /// Internal data, do not touch
    pub private: *mut c_void,
}
