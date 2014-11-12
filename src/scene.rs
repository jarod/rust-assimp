//! Defines the data structures in which the imported scene is returned.

use animation::Animation;
use camera::Camera;
use light::Light;
use material::Material;
use mesh::Mesh;
use texture::Texture;
use types::{Matrix4x4, aiString};
use libc::{c_uint, c_void};

    /** A node in the imported hierarchy.
     *
     * Each node has name, a parent node (except for the root node),
     * a transformation relative to its parent and possibly several child nodes.
     * Simple file formats don't support hierarchical structures - for these formats
     * the imported scene does consist of only a single root node without children.
     */
#[repr(C)]
pub struct Node {
    /** The name of the node.
        *
        * The name might be empty (length of zero) but all nodes which
        * need to be accessed afterwards by bones or anims are usually named.
        * Multiple nodes may have the same name, but nodes which are accessed
        * by bones (see #aiBone and #aiMesh::mBones) *must* be unique.
        *
        * Cameras and lights are assigned to a specific node name - if there
        * are multiple nodes with this name, they're assigned to each of them.
        * <br>
        * There are no limitations regarding the characters contained in
        * this text. You should be able to handle stuff like whitespace, tabs,
        * linefeeds, quotation marks, ampersands, ... .
        */
    mName: aiString,

    /** The transformation relative to the node's parent. */
    mTransformation: Matrix4x4,

    /** Parent node. NULL if this node is the root node. */
    mParent: *mut Node,

    /** The number of child nodes of this node. */
    mNumChildren: c_uint,

    /** The child nodes of this node. NULL if mNumChildren is 0. */
    mChildren: *mut*mut Node,

    /** The number of meshes of this node. */
    mNumMeshes: c_uint,

    /** The meshes of this node. Each entry is an index into the mesh */
    mMeshes: *mut c_uint,
}

impl Node {
    pub fn get_name(&self) -> Option<&str> {
        self.mName.as_str()
    }
    pub fn get_num_children(&self) -> u32 {
        self.mNumChildren
    }
    pub fn get_num_meshes(&self) -> u32 {
        self.mNumMeshes
    }
    pub fn get_trans_matrix(&self) -> &Matrix4x4 {
        &self.mTransformation
    }
}


/** @def AI_SCENE_FLAGS_INCOMPLETE
    * Specifies that the scene data structure that was imported is not complete.
    * This flag bypasses some internal validations and allows the import
    * of animation skeletons, material libraries or camera animation paths
    * using Assimp. Most applications won't support such data.
    */
const AI_SCENE_FLAGS_INCOMPLETE: c_uint = 0x1;

/** @def AI_SCENE_FLAGS_VALIDATED
    * This flag is set by the validation postprocess-step (aiPostProcess_ValidateDS)
    * if the validation is successful. In a validated scene you can be sure that
    * any cross references in the data structure (e.g. vertex indices) are valid.
    */
const AI_SCENE_FLAGS_VALIDATED: c_uint = 0x2;

/** @def AI_SCENE_FLAGS_VALIDATION_WARNING
    * This flag is set by the validation postprocess-step (aiPostProcess_ValidateDS)
    * if the validation is successful but some issues have been found.
    * This can for example mean that a texture that does not exist is referenced
    * by a material or that the bone weights for a vertex don't sum to 1.0 ... .
    * In most cases you should still be able to use the import. This flag could
    * be useful for applications which don't capture Assimp's log output.
    */
const AI_SCENE_FLAGS_VALIDATION_WARNING: c_uint = 0x4;

/** @def AI_SCENE_FLAGS_NON_VERBOSE_FORMAT
    * This flag is currently only set by the aiProcess_JoinIdenticalVertices step.
    * It indicates that the vertices of the output meshes aren't in the internal
    * verbose format anymore. In the verbose format all vertices are unique,
    * no vertex is ever referenced by more than one face.
    */
const AI_SCENE_FLAGS_NON_VERBOSE_FORMAT: c_uint = 0x8;

/** @def AI_SCENE_FLAGS_TERRAIN
    * Denotes pure height-map terrain data. Pure terrains usually consist of quads,
    * sometimes triangles, in a regular grid. The x,y coordinates of all vertex
    * positions refer to the x,y coordinates on the terrain height map, the z-axis
    * stores the elevation at a specific point.
    *
    * TER (Terragen) and HMP (3D Game Studio) are height map formats.
    * @note Assimp is probably not the best choice for loading *huge* terrains -
    * fully triangulated data takes extremely much free store and should be avoided
    * as long as possible (typically you'll do the triangulation when you actually
    * need to render it).
    */
const AI_SCENE_FLAGS_TERRAIN : c_uint = 0x10;


/** The root structure of the imported data.
    *
    *  Everything that was imported from the given file can be accessed from here.
    *  Objects of this class are generally maintained and owned by Assimp, not
    *  by the caller. You shouldn't want to instance it, nor should you ever try to
    *  delete a given scene on your own.
    */
#[repr(C)]
pub struct Scene {
    /** Any combination of the AI_SCENE_FLAGS_XXX flags. By default
        * this value is 0, no flags are set. Most applications will
        * want to reject all scenes with the AI_SCENE_FLAGS_INCOMPLETE
        * bit set.
        */
    mFlags: c_uint,

    /** The root node of the hierarchy.
        *
        * There will always be at least the root node if the import
        * was successful (and no special flags have been set).
        * Presence of further nodes depends on the format and content
        * of the imported file.
        */
    mRootNode: *mut Node,

    /** The number of meshes in the scene. */
    mNumMeshes: c_uint,

    /** The array of meshes.
        *
        * Use the indices given in the aiNode structure to access
        * this array. The array is mNumMeshes in size. If the
        * AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
        * be at least ONE material.
        */
    mMeshes: *mut*mut Mesh,

    /** The number of materials in the scene. */
    mNumMaterials: c_uint,

    /** The array of materials.
        *
        * Use the index given in each aiMesh structure to access this
        * array. The array is mNumMaterials in size. If the
        * AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
        * be at least ONE material.
        */
    mMaterials: *mut*mut Material,

    /** The number of animations in the scene. */
    mNumAnimations: c_uint,

    /** The array of animations.
        *
        * All animations imported from the given file are listed here.
        * The array is mNumAnimations in size.
        */
    mAnimations: *mut*mut Animation,

    /** The number of textures embedded into the file */
    mNumTextures: c_uint,

    /** The array of embedded textures.
        *
        * Not many file formats embed their textures into the file.
        * An example is Quake's MDL format (which is also used by
        * some GameStudio versions)
        */
    mTextures: *mut*mut Texture,


    /** The number of light sources in the scene. Light sources
        * are fully optional, in most cases this attribute will be 0
        */
    mNumLights: c_uint,

    /** The array of light sources.
        *
        * All light sources imported from the given file are
        * listed here. The array is mNumLights in size.
        */
    mLights: Light,


    /** The number of cameras in the scene. Cameras
        * are fully optional, in most cases this attribute will be 0
        */
    mNumCameras: c_uint,

    /** The array of cameras.
        *
        * All cameras imported from the given file are listed here.
        * The array is mNumCameras in size. The first camera in the
        * array (if existing) is the default camera view into
        * the scene.
        */
    mCameras: *mut*mut Camera,

    /**  Internal data, do not touch */
    mPrivate: *mut c_void,
}

impl Scene {
    // Change access method to iterator
    pub fn get_root_node(&self) -> &Node {
        unsafe {
            &*self.mRootNode
        }
    }
    pub fn get_num_animations(&self) -> u32 {
        self.mNumAnimations
    }
    pub fn get_num_cameras(&self) -> u32 {
        self.mNumCameras
    }
    pub fn get_num_lights(&self) -> u32 {
        self.mNumLights
    }
    pub fn get_num_materials(&self) -> u32 {
        self.mNumMaterials
    }
    pub fn get_num_meshes(&self) -> u32 {
        self.mNumMeshes
    }
    pub fn get_num_textures(&self) -> u32 {
        self.mNumTextures
    }
}
