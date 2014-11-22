//! Declares the data structures used for imported geometry.

use libc::{c_uint, c_float};
use std::fmt;

use types::{Vector3D, Color4D, Matrix4x4, AiString};
use util::{ptr_ptr_to_slice, ptr_to_slice};

use mesh::PrimitiveType::{Point, Line, Triangle, Polygon};

/// Maximum number of indices per face (polygon).
pub const MAX_FACE_INDICES : uint = 0x7fff;

/// Maximum number of indices per face (polygon).
pub const MAX_BONE_WEIGHTS : uint = 0x7fffffff;

/// Maximum number of vertices per mesh.
pub const MAX_VERTICES : uint = 0x7fffffff;

/// Maximum number of faces per mesh.
pub const MAX_FACES : uint = 0x7fffffff;

/// Supported number of vertex color sets per mesh.
pub const MAX_NUMBER_OF_COLOR_SETS : uint = 0x8;

/// Supported number of texture coord sets (uv[w] channels) per mesh
pub const MAX_NUMBER_OF_TEXTURECOORDS : uint = 0x8;

/// A single face in a mesh, referring to multiple vertices.
///
/// If num_indices is 3, we call the face 'triangle', for num_indices > 3
/// it's classed as a 'polygon'.
///
/// Mesh::primitive_types can be queried to quickly examine which types of
/// primitive are actually present in a mesh. The `Process_SortByPType` flag
/// executes a special post-processing algorithm which splits meshes with
/// *different* primitive types mixed up (e.g. lines and triangles) in several
/// 'clean' submeshes. Furthermore there is a configuration option
/// `PP_SBP_REMOVE` to force `Process_SortByPType` to remove
/// specific kinds of primitives from the imported scene, completely and forever.
/// In many cases you'll probably want to set this setting to
/// ` PrimitiveType_LINE | PrimitiveType_POINT `
/// Together with the `Process_Triangulate` flag you can then be sure that
/// Face::num_indices is always 3.
#[repr(C)]
pub struct Face {
    /// Number of indices defining this face.
    ///
    /// The maximum value for this member is `MAX_FACE_INDICES`.
    pub num_indices: c_uint,

    /// Pointer to the indices array. Size of the array is given in numIndices.
    indices: *mut c_uint,
}

impl Face {
    /// Get the list of indices referenced by this face.
    pub fn get_indices(&self) -> &[u32] {
        unsafe { ptr_to_slice(self.indices, self.num_indices as uint) }
    }
}

/// A single influence of a bone on a vertex.
#[deriving(Show)]
#[repr(C)]
pub struct VertexWeight {
    /// Index of the vertex which is influenced by the bone.
    pub vertex_id: c_uint,

    /// The strength of the influence in the range (0...1).
    ///
    /// The influence from all bones at one vertex amounts to 1.
    pub weight: c_float,
}

/// A single bone of a mesh.
///
/// A bone has a name by which it can be found in the frame hierarchy and by
/// which it can be addressed by animations. In addition it has a number of
/// influences on vertices.
#[repr(C)]
pub struct Bone {
    /// The name of the bone.
    pub name: AiString,

    /// The number of vertices affected by this bone
    ///
    /// The maximum value for this member is `MAX_BONE_WEIGHTS`.
    pub num_weights: c_uint,

    /// The vertices affected by this bone
    weights: *mut VertexWeight,

    /// Matrix that transforms from mesh space to bone space in bind pose
    pub offset_matrix: Matrix4x4,
}

impl Bone {
    /// Get the vertices affected by this bone
    pub fn get_weights(&self) -> &[VertexWeight] {
        unsafe { ptr_to_slice(self.weights, self.num_weights as uint) }
    }
}

/// Enumerates the types of geometric primitives supported by Assimp.
#[repr(C)]
pub enum PrimitiveType {
    /// A point primitive.
    Point = 0x1,

    /// A line primitive.
    Line = 0x2,

    /// A triangular primitive.
    Triangle = 0x4,

    /// A higher-level polygon with more than 3 edges.
    Polygon = 0x8,
}

impl PrimitiveType {
    /// Create a new primitive type from the given number of vertices
    pub fn get_primitive_type(n: u32) -> PrimitiveType {
        match n {
            0 => panic!("0 is an invalid number for vertices in a face"),
            1 => Point,
            2 => Line,
            3 => Triangle,
            _ => Polygon,
        }
    }
}

/// NOT CURRENTLY IN USE. An AnimMesh is an attachment to a Mesh that stores per-vertex
/// animations for a particular frame.
///
/// You may think of an AnimMesh as a `patch` for the host mesh, which
/// replaces only certain vertex data streams at a particular time.  Each mesh
/// stores n attached attached meshes (Mesh::anim_meshes).  The actual
/// relationship between the time line and anim meshes is established by
/// MeshAnim, which references singular mesh attachments by their ID and binds
/// them to a time offset.
#[repr(C)]
pub struct AnimMesh {
    /// Replacement for Mesh vertices.
    ///
    /// If this array is non-NULL, it *must* contain mNumVertices entries.
    /// The corresponding array in the host mesh must be non-NULL as well -
    /// animation meshes may neither add or nor remove vertex components (if a
    /// replacement array is NULL and the corresponding source array is not,
    /// the source data is taken instead)
    vertices: *mut Vector3D,

    /// Replacement for Mesh normals.
    normals: *mut Vector3D,

    /// Replacement for Mesh tangents.
    tangents: *mut Vector3D,

    /// Replacement for Mesh bitangents.
    bitangents: *mut Vector3D,

    /// Replacement for Mesh colors
    colors: [*mut Color4D, ..MAX_NUMBER_OF_COLOR_SETS],

    /// Replacement for Mesh texture_coords
    texture_coords: [*mut Vector3D, ..MAX_NUMBER_OF_TEXTURECOORDS],

    /// The number of vertices in the AnimMesh, and thus the length of all
    /// the member arrays.
    ///
    /// This has always the same value as the num_vertices property in the
    /// corresponding Mesh. It is duplicated here merely to make the length
    /// of the member arrays accessible even if the Mesh is not known, e.g.
    /// from language bindings.
    pub num_vertices: c_uint,
}

impl AnimMesh {
    /// Replacement for Mesh vertices.
    ///
    /// If the array is present, it *must* contain num_vertices entries.
    /// The corresponding array in the host mesh must not be `None` as well -
    /// animation meshes may neither add or nor remove vertex components (if a
    /// replacement array is `None` and the corresponding source array is not,
    /// the source data is taken instead)
    pub fn get_vertices(&self) -> &[Vector3D] {
        unsafe { ptr_to_slice(self.vertices, self.num_vertices as uint) }
    }

    /// Replacement for Mesh normals.
    pub fn get_normals(&self) -> &[Vector3D] {
        let len = match self.normals.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.normals, len) }
    }

    /// Replacement for Mesh tangents.
    pub fn get_tangents(&self) -> &[Vector3D] {
        let len = match self.tangents.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.tangents, len) }
    }

    /// Replacement for Mesh bitangents.
    pub fn get_bitangents(&self) -> &[Vector3D] {
        let len = match self.bitangents.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.bitangents, len) }
    }

    /// Replacement for Mesh colors
    pub fn get_colors(&self) -> Vec<&[Color4D]> {
        let mut list = Vec::with_capacity(MAX_NUMBER_OF_COLOR_SETS);

        for colors in self.colors.iter() {
            if colors.is_null() { break; }
            unsafe {
                list.push(ptr_to_slice(*colors, self.num_vertices as uint));
            }
        }

        list
    }

    /// Vertex texture coords, also known as UV channels.
    ///
    /// A mesh may contain 0 to `MAX_NUMBER_OF_TEXTURECOORDS` per
    /// vertex. `None` if not present. The array is num_vertices in size.
    /// Replacement for Mesh texture_coords
    pub fn get_texture_coords(&self) -> Vec<&[Vector3D]> {
        let mut list = Vec::with_capacity(MAX_NUMBER_OF_TEXTURECOORDS);

        for tex_coords in self.texture_coords.iter() {
            if tex_coords.is_null() { break; }
            unsafe {
                list.push(ptr_to_slice(*tex_coords, self.num_vertices as uint));
            }
        }

        list
    }
}

/// A mesh represents a geometry or model with a single material.
///
/// It usually consists of a number of vertices and a series of
/// primitives/faces referencing the vertices. In addition there might be a
/// series of bones, each of them addressing a number of vertices with a
/// certain weight. Vertex data is presented in channels with each channel
/// containing a single per-vertex information such as a set of texture coords
/// or a normal vector.  If a data pointer is non-null, the corresponding data
/// stream is present.  From C++-programs you can also use the comfort
/// functions Has*() to test for the presence of various data streams.
///
/// A Mesh uses only a single material which is referenced by a material ID.
///
/// Note: The positions field is usually not optional. However, vertex
/// positions *could* be missing if the `SceneFlags::Incomplete` flag is set
/// in `Scene::flags`.
#[repr(C)]
pub struct Mesh {
    /// Bitwise combination of the members of the PrimitiveType enum.
    ///
    /// This specifies which types of primitives are present in the mesh.
    /// The "SortByPrimitiveType"-Step can be used to make sure the
    /// output meshes consist of one primitive type each.
    pub primitive_types: c_uint,

    /// The number of vertices in this mesh.
    ///
    /// This is also the size of all of the per-vertex data arrays.
    /// The maximum value for this member is `MAX_VERTICES`.
    pub num_vertices: c_uint,

    /// The number of primitives (triangles, polygons, lines) in this  mesh.
    /// This is also the size of the faces array.
    /// The maximum value for this member is `MAX_FACES`.
    pub num_faces: c_uint,

    /// Vertex positions.
    ///
    /// This array is always present in a mesh. The array is
    /// num_vertices in size.
    vertices: *mut Vector3D,

    /// Vertex normals.
    ///
    /// The array contains normalized vectors, NULL if not present.
    /// The array is num_vertices in size. Normals are undefined for
    /// point and line primitives. A mesh consisting of points and
    /// lines only may not have normal vectors. Meshes with mixed
    /// primitive types (i.e. lines and triangles) may have normals,
    /// but the normals for vertices that are only referenced by
    /// point or line primitives are undefined and set to QNaN (WARN:
    /// qNaN compares to inequal to *everything*, even to qNaN itself.
    /// Using code like this to check whether a field is qnan is:
    ///
    /// ```c
    /// #define IS_QNAN(f) (f != f)
    /// ```
    ///
    /// still dangerous because even 1.f == 1.f could evaluate to false! (
    /// remember the subtleties of IEEE754 artithmetics). Use stuff like
    /// fpclassify instead.
    ///
    /// Note: Normal vectors computed by Assimp are always unit-length.
    /// However, this needn't apply for normals that have been taken directly
    /// from the model file.
    normals: *mut Vector3D,

    /// Vertex tangents.
    ///
    /// The tangent of a vertex points in the direction of the positive
    /// X texture axis. The array contains normalized vectors, NULL if
    /// not present. The array is mNumVertices in size. A mesh consisting
    /// of points and lines only may not have normal vectors. Meshes with
    /// mixed primitive types (i.e. lines and triangles) may have
    /// normals, but the normals for vertices that are only referenced by
    /// point or line primitives are undefined and set to qNaN.  See
    /// the normals member for a detailled discussion of qNaNs.
    ///
    /// Note: If the mesh contains tangents, it automatically also
    /// contains bitangents.
    tangents: *mut Vector3D,

    /// Vertex bitangents.
    ///
    /// The bitangent of a vertex points in the direction of the positive
    /// Y texture axis. The array contains normalized vectors, NULL if not
    /// present. The array is num_vertices in size.
    ///
    /// Note: If the mesh contains tangents, it automatically also contains
    /// bitangents.
    bitangents : *mut Vector3D,

    /// Vertex color sets.
    ///
    /// A mesh may contain 0 to `MAX_NUMBER_OF_COLOR_SETS` vertex colors per
    /// vertex. NULL if not present. Each array is num_vertices in size if
    /// present.
    colors: [*mut Color4D, ..MAX_NUMBER_OF_COLOR_SETS],

    /// Vertex texture coords, also known as UV channels.
    ///
    /// A mesh may contain 0 to `MAX_NUMBER_OF_TEXTURECOORDS` per
    /// vertex. NULL if not present. The array is mNumVertices in size.
    texture_coords: [*mut Vector3D, ..MAX_NUMBER_OF_TEXTURECOORDS],

    /// Specifies the number of components for a given UV channel.
    ///
    /// Up to three channels are supported (UVW, for accessing volume
    /// or cube maps). If the value is 2 for a given channel n, the
    /// component p.z of mTextureCoords[n][p] is set to 0.0f.
    /// If the value is 1 for a given channel, p.y is set to 0.0f, too.
    ///
    /// Note: 4D coords are not supported
    pub num_uv_components: [c_uint, ..MAX_NUMBER_OF_TEXTURECOORDS],

    /// The faces the mesh is constructed from.
    ///
    /// Each face refers to a number of vertices by their indices.
    /// This array is always present in a mesh, its size is given
    /// in mNumFaces. If the SceneFlags::NON_VERBOSE_FORMAT
    /// is NOT set each face references an unique set of vertices.
    faces: *mut Face,

    /// The number of bones this mesh contains.
    ///
    /// Can be 0, in which case the bones array is NULL.
    pub num_bones: c_uint,

    /// The bones of this mesh.
    ///
    /// A bone consists of a name by which it can be found in the
    /// frame hierarchy and a set of vertex weights.
    bones: *mut*mut Bone,

    /// The material used by this mesh.
    ///
    /// A mesh does use only a single material. If an imported model uses
    /// multiple materials, the import splits up the mesh. Use this value
    /// as index into the scene's material list.
    pub material_index: c_uint,

    /// Name of the mesh.
    ///
    /// Meshes can be named, but this is not a requirement and leaving this
    /// field empty is totally fine.  There are mainly three uses for mesh
    /// names:
    ///
    ///  * some formats name nodes and meshes independently.
    ///
    ///  * importers tend to split meshes up to meet the
    ///     one-material-per-mesh requirement. Assigning
    ///     the same (dummy) name to each of the result meshes
    ///     aids the caller at recovering the original mesh
    ///     partitioning.
    ///
    ///  * Vertex animations refer to meshes by their names.
    pub name: AiString,

    /// NOT CURRENTLY IN USE. The number of attachment meshes.
    pub num_anim_meshes: c_uint,

    /// NOT CURRENTLY IN USE. Attachment meshes for this mesh, for vertex-based animation.
    /// Attachment meshes carry replacement data for some of the mesh'es
    /// vertex components (usually positions, normals).
    anim_meshes: *mut*mut AnimMesh,
}

impl Mesh {
    /// Check if the mesh has a given primitive type
    pub fn has_primitive(&self, prim: PrimitiveType) -> bool {
        (self.primitive_types & prim as u32) != 0
    }

    /// Vertex positions.
    ///
    /// This array is always present in a mesh. The array is
    /// num_vertices in size.
    pub fn get_vertices(&self) -> &[Vector3D] {
        unsafe { ptr_to_slice(self.vertices, self.num_vertices as uint) }
    }

    /// Vertex normals.
    ///
    /// The array contains normalized vectors, NULL if not present.
    /// The array is num_vertices in size. Normals are undefined for
    /// point and line primitives. A mesh consisting of points and
    /// lines only may not have normal vectors. Meshes with mixed
    /// primitive types (i.e. lines and triangles) may have normals,
    /// but the normals for vertices that are only referenced by
    /// point or line primitives are undefined and set to QNaN (WARN:
    /// qNaN compares to inequal to *everything*, even to qNaN itself.
    /// Using code like this to check whether a field is qnan is:
    ///
    /// ```c
    /// is_qnan(f) (f != f)
    /// ```
    ///
    /// still dangerous because even 1.f == 1.f could evaluate to false! (
    /// remember the subtleties of IEEE754 artithmetics).    ///
    ///
    /// Note: Normal vectors computed by Assimp are always unit-length.
    /// However, this needn't apply for normals that have been taken directly
    /// from the model file.
    pub fn get_normals(&self) -> &[Vector3D] {
        let len = match self.normals.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.normals, len) }
    }

    /// Vertex tangents.
    ///
    /// The tangent of a vertex points in the direction of the positive
    /// X texture axis. The array contains normalized vectors. Returns
    /// `None` if not present. The array is num_vertices in size. A mesh
    /// consisting of points and lines only may not have tangent vectors.
    /// Meshes with mixed primitive types (i.e. lines and triangles) may have
    /// tangents, but the tangents for vertices that are only referenced by
    /// point or line primitives are undefined and set to qNaN.  See the
    /// normals member for a detailled discussion of qNaNs.
    ///
    /// Note: If the mesh contains tangents, it automatically also
    /// contains bitangents.
    pub fn get_tangents(&self) -> &[Vector3D] {
        let len = match self.tangents.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.tangents, len) }
    }

    /// Vertex bitangents.
    ///
    /// The bitangent of a vertex points in the direction of the positive
    /// Y texture axis. The array contains normalized vectors, NULL if not
    /// present. The array is num_vertices in size.
    ///
    /// Note: If the mesh contains tangents, it automatically also contains
    /// bitangents.
    pub fn get_bitangents(&self) -> &[Vector3D] {
        let len = match self.bitangents.is_null() {
            true => 0,
            false => self.num_vertices as uint,
        };
        unsafe { ptr_to_slice(self.bitangents, len) }
    }

    /// Vertex color sets.
    ///
    /// A mesh may contain 0 to `MAX_NUMBER_OF_COLOR_SETS` vertex colors per
    /// vertex. `None` if not present. Each array is num_vertices in size if
    /// present.
    pub fn get_colors(&self) -> Vec<&[Color4D]> {
        let mut list = Vec::with_capacity(MAX_NUMBER_OF_COLOR_SETS);

        for colors in self.colors.iter() {
            if colors.is_null() { break; }
            unsafe {
                list.push(ptr_to_slice(*colors, self.num_vertices as uint));
            }
        }

        list
    }

    /// Vertex texture coords, also known as UV channels.
    ///
    /// A mesh may contain 0 to `MAX_NUMBER_OF_TEXTURECOORDS` per
    /// vertex. `None` if not present. The array is num_vertices in size.
    pub fn get_texture_coords(&self) -> Vec<&[Vector3D]> {
        let mut list = Vec::with_capacity(MAX_NUMBER_OF_COLOR_SETS);

        for tex_coords in self.texture_coords.iter() {
            if tex_coords.is_null() { break; }
            unsafe {
                list.push(ptr_to_slice(*tex_coords, self.num_vertices as uint));
            }
        }

        list
    }

    /// The faces the mesh is constructed from.
    ///
    /// Each face refers to a number of vertices by their indices.
    /// This array is always present in a mesh.
    /// If the `SceneFlags::NonVerboseFormat` is *not* set each face references
    /// an unique set of vertices.
    pub fn get_faces(&self) -> &[Face] {
        unsafe { ptr_to_slice(self.faces, self.num_faces as uint) }
    }

    /// The bones of this mesh.
    ///
    /// A bone consists of a name by which it can be found in the frame
    /// hierarchy and a set of vertex weights.
    pub fn get_bones(&self) -> &[&Bone] {
        unsafe { ptr_ptr_to_slice(self.bones, self.num_bones as uint) }
    }
}

impl fmt::Show for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Mesh {{ name: {}, primitive_types:", self.name,));

        if self.has_primitive(Point) { try!(write!(f, " Point")) }
        if self.has_primitive(Line) { try!(write!(f, " Line")) }
        if self.has_primitive(Triangle) { try!(write!(f, " Triangle")) }
        if self.has_primitive(Polygon) { try!(write!(f, " Polygon")) }

        write!(f, ", num_vertices: {}, num_faces: {}, num_bones: {}, material_index: {} }}",
        self.num_vertices,
        self.num_faces,
        self.num_bones,
        self.material_index)
    }
}

// vim: et tw=78 sw=4:
