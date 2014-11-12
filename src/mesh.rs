//! Declares the data structures in which the imported geometry is
//! returned by ASSIMP: Mesh, Face and Bone data structures.

use types::{Vector3D, Color4D, Matrix4x4, aiString};
use libc::{c_uint, c_float};

/// Maximum number of indices per face (polygon).
const AI_MAX_FACE_INDICES : uint = 0x7fff;

/// Maximum number of indices per face (polygon).
const AI_MAX_BONE_WEIGHTS : uint = 0x7fffffff;

/// Maximum number of vertices per mesh.
const AI_MAX_VERTICES : uint = 0x7fffffff;

/// Maximum number of faces per mesh.
const AI_MAX_FACES : uint = 0x7fffffff;

/// Supported number of vertex color sets per mesh.
const AI_MAX_NUMBER_OF_COLOR_SETS : uint = 0x8;

/// Supported number of texture coord sets (UV(W) channels) per mesh
const AI_MAX_NUMBER_OF_TEXTURECOORDS : uint = 0x8;

/// A single face in a mesh, referring to multiple vertices.
///
// /// If mNumIndices is 3, we call the face 'triangle', for mNumIndices > 3
// /// it's called 'polygon' (hey, that's just a definition!).
// ///
// /// aiMesh::mPrimitiveTypes can be queried to quickly examine which types of
// /// primitive are actually present in a mesh. The #aiProcess_SortByPType flag
// /// executes a special post-processing algorithm which splits meshes with
// /// *different* primitive types mixed up (e.g. lines and triangles) in several
// /// 'clean' submeshes. Furthermore there is a configuration option (
// /// #AI_CONFIG_PP_SBP_REMOVE) to force #aiProcess_SortByPType to remove
// /// specific kinds of primitives from the imported scene, completely and forever.
// /// In many cases you'll probably want to set this setting to
// /// @code
// /// aiPrimitiveType_LINE|aiPrimitiveType_POINT
// /// @endcode
// /// Together with the #aiProcess_Triangulate flag you can then be sure that
// /// #aiFace::mNumIndices is always 3.
// /// @note Take a look at the @link data Data Structures page @endlink for
// /// more information on the layout and winding order of a face.
// ///
#[repr(C)]
pub struct Face {
    /// Number of indices defining this face.
    // ///
    // /// The maximum value for this member is Limits::AI_MAX_FACE_INDICES.
    // unsigned int mNumIndices;
    mNumIndices: c_uint,

    /// Pointer to the indices array. Size of the array is given in numIndices.
    // unsigned int* mIndices;
    mIndices: *mut c_uint,
}


/// A single influence of a bone on a vertex.
#[repr(C)]
pub struct VertexWeight {
    /// Index of the vertex which is influenced by the bone.
    // unsigned int mVertexId;
    mVertexId: c_uint,

    /// The strength of the influence in the range (0...1).
    ///
    /// The influence from all bones at one vertex amounts to 1.
    // float mWeight;
    mWeight: c_float,
}

/// A single bone of a mesh.
///
/// A bone has a name by which it can be found in the frame hierarchy and by
/// which it can be addressed by animations. In addition it has a number of
/// influences on vertices.
#[repr(C)]
pub struct Bone {
    /// The name of the bone.
    // C_STRUCT aiString mName;
    mName: aiString,

    /// The number of vertices affected by this bone
    /// The maximum value for this member is #AI_MAX_BONE_WEIGHTS.
    // unsigned int mNumWeights;
    mNumWeights: c_uint,

    /// The vertices affected by this bone
    // C_STRUCT aiVertexWeight* mWeights;
    mWeights: *mut VertexWeight,

    /// Matrix that transforms from mesh space to bone space in bind pose
    // C_STRUCT aiMatrix4x4 mOffsetMatrix;
    mOffsetMatrix: Matrix4x4,
}

/// Enumerates the types of geometric primitives supported by Assimp.
///
#[repr(C, u32)]
pub enum PrimitiveType {
    /// A point primitive.
    POINT = 0x1,

    /// A line primitive.
    LINE = 0x2,

    /// A triangular primitive.
    TRIANGLE = 0x4,

    /// A higher-level polygon with more than 3 edges.
    POLYGON = 0x8,
}

    // // Get the #aiPrimitiveType flag for a specific number of face indices
// #define AI_PRIMITIVE_TYPE_FOR_N_INDICES(n) \
    // ((n) > 3 ? aiPrimitiveType_POLYGON : (aiPrimitiveType)(1u << ((n)-1)))



    /// NOT CURRENTLY IN USE. An AnimMesh is an attachment to a Mesh that stores per-vertex
    /// animations for a particular frame.
    ///
    // /// You may think of an AnimMesh as a `patch` for the host mesh, which
    // /// replaces only certain vertex data streams at a particular time.
    // /// Each mesh stores n attached attached meshes (#aiMesh::mAnimMeshes).
    // /// The actual relationship between the time line and anim meshes is
    // /// established by #aiMeshAnim, which references singular mesh attachments
    // /// by their ID and binds them to a time offset.
    // ///
#[repr(C)]
pub struct AnimMesh {
    // /** Replacement for aiMesh::mVertices. If this array is non-NULL,
    //  *  it *must* contain mNumVertices entries. The corresponding
    //  *  array in the host mesh must be non-NULL as well - animation
    //  *  meshes may neither add or nor remove vertex components (if
    //  *  a replacement array is NULL and the corresponding source
    //  *  array is not, the source data is taken instead)*/
    // C_STRUCT aiVector3D* mVertices;
    mVertices: *mut Vector3D,

    /** Replacement for aiMesh::mNormals.  */
    // C_STRUCT aiVector3D* mNormals;
    mNormals: *mut Vector3D,

    /** Replacement for aiMesh::mTangents. */
    // C_STRUCT aiVector3D* mTangents;,
    mTangents: *mut Vector3D,

    /** Replacement for aiMesh::mBitangents. */
    // C_STRUCT aiVector3D* mBitangents;
    mBitangents: *mut Vector3D,

    /** Replacement for aiMesh::mColors */
    // C_STRUCT aiColor4D* mColors[AI_MAX_NUMBER_OF_COLOR_SETS];
    mColors: *mut [Color4D, ..AI_MAX_NUMBER_OF_COLOR_SETS],

    /** Replacement for aiMesh::mTextureCoords */
    // C_STRUCT aiVector3D* mTextureCoords[AI_MAX_NUMBER_OF_TEXTURECOORDS];
    mTextureCoords: *mut [Vector3D, ..AI_MAX_NUMBER_OF_TEXTURECOORDS],

    /** The number of vertices in the aiAnimMesh, and thus the length of all
        * the member arrays.
        *
        * This has always the same value as the mNumVertices property in the
        * corresponding aiMesh. It is duplicated here merely to make the length
        * of the member arrays accessible even if the aiMesh is not known, e.g.
        * from language bindings.
        */
    // unsigned int mNumVertices;
    mNumVertices: c_uint,
}


/** @brief A mesh represents a geometry or model with a single material.
    *
    * It usually consists of a number of vertices and a series of primitives/faces
    * referencing the vertices. In addition there might be a series of bones, each
    * of them addressing a number of vertices with a certain weight. Vertex data
    * is presented in channels with each channel containing a single per-vertex
    * information such as a set of texture coords or a normal vector.
    * If a data pointer is non-null, the corresponding data stream is present.
    * From C++-programs you can also use the comfort functions Has*() to
    * test for the presence of various data streams.
    *
    * A Mesh uses only a single material which is referenced by a material ID.
    * @note The mPositions member is usually not optional. However, vertex positions
    * *could* be missing if the #AI_SCENE_FLAGS_INCOMPLETE flag is set in
    * @code
    * aiScene::mFlags
    * @endcode
    */
#[repr(C)]
pub struct Mesh {
    /** Bitwise combination of the members of the #aiPrimitiveType enum.
        * This specifies which types of primitives are present in the mesh.
        * The "SortByPrimitiveType"-Step can be used to make sure the
        * output meshes consist of one primitive type each.
        */
    // unsigned int mPrimitiveTypes;
    mPrimitiveTypes: c_uint,

    /** The number of vertices in this mesh.
        * This is also the size of all of the per-vertex data arrays.
        * The maximum value for this member is #AI_MAX_VERTICES.
        */
    // unsigned int mNumVertices;
    mNumVertices: c_uint,

    /** The number of primitives (triangles, polygons, lines) in this  mesh.
        * This is also the size of the mFaces array.
        * The maximum value for this member is #AI_MAX_FACES.
        */
    // unsigned int mNumFaces;
    mNumFaces: c_uint,

    /** Vertex positions.
        * This array is always present in a mesh. The array is
        * mNumVertices in size.
        */
    // C_STRUCT aiVector3D* mVertices;
    mVertices: *mut Vector3D,

    /** Vertex normals.
        * The array contains normalized vectors, NULL if not present.
        * The array is mNumVertices in size. Normals are undefined for
        * point and line primitives. A mesh consisting of points and
        * lines only may not have normal vectors. Meshes with mixed
        * primitive types (i.e. lines and triangles) may have normals,
        * but the normals for vertices that are only referenced by
        * point or line primitives are undefined and set to QNaN (WARN:
        * qNaN compares to inequal to *everything*, even to qNaN itself.
        * Using code like this to check whether a field is qnan is:
        * @code
        * #define IS_QNAN(f) (f != f)
        * @endcode
        * still dangerous because even 1.f == 1.f could evaluate to false! (
        * remember the subtleties of IEEE754 artithmetics). Use stuff like
        * @c fpclassify instead.
        * @note Normal vectors computed by Assimp are always unit-length.
        * However, this needn't apply for normals that have been taken
        *   directly from the model file.
        */
    // C_STRUCT aiVector3D* mNormals;
    mNormals: *mut Vector3D,

    /** Vertex tangents.
        * The tangent of a vertex points in the direction of the positive
        * X texture axis. The array contains normalized vectors, NULL if
        * not present. The array is mNumVertices in size. A mesh consisting
        * of points and lines only may not have normal vectors. Meshes with
        * mixed primitive types (i.e. lines and triangles) may have
        * normals, but the normals for vertices that are only referenced by
        * point or line primitives are undefined and set to qNaN.  See
        * the #mNormals member for a detailled discussion of qNaNs.
        * @note If the mesh contains tangents, it automatically also
        * contains bitangents.
        */
    // C_STRUCT aiVector3D* mTangents;
    mTangents: *mut Vector3D,

    /** Vertex bitangents.
        * The bitangent of a vertex points in the direction of the positive
        * Y texture axis. The array contains normalized vectors, NULL if not
        * present. The array is mNumVertices in size.
        * @note If the mesh contains tangents, it automatically also contains
        * bitangents.
        */
    // C_STRUCT aiVector3D* mBitangents;
    mBitangents : *mut Vector3D,

    /** Vertex color sets.
        * A mesh may contain 0 to #AI_MAX_NUMBER_OF_COLOR_SETS vertex
        * colors per vertex. NULL if not present. Each array is
        * mNumVertices in size if present.
        */
    // C_STRUCT aiColor4D* mColors[AI_MAX_NUMBER_OF_COLOR_SETS];
    mColors: *mut [Color4D, ..AI_MAX_NUMBER_OF_COLOR_SETS],

    /** Vertex texture coords, also known as UV channels.
        * A mesh may contain 0 to AI_MAX_NUMBER_OF_TEXTURECOORDS per
        * vertex. NULL if not present. The array is mNumVertices in size.
        */
    // C_STRUCT aiVector3D* mTextureCoords[AI_MAX_NUMBER_OF_TEXTURECOORDS];
    mTextureCoords: *mut [Vector3D, ..AI_MAX_NUMBER_OF_TEXTURECOORDS],

    /** Specifies the number of components for a given UV channel.
        * Up to three channels are supported (UVW, for accessing volume
        * or cube maps). If the value is 2 for a given channel n, the
        * component p.z of mTextureCoords[n][p] is set to 0.0f.
        * If the value is 1 for a given channel, p.y is set to 0.0f, too.
        * @note 4D coords are not supported
        */
    // unsigned int mNumUVComponents[AI_MAX_NUMBER_OF_TEXTURECOORDS];
    mNumUVComponents: [c_uint, ..AI_MAX_NUMBER_OF_TEXTURECOORDS],

    /** The faces the mesh is constructed from.
        * Each face refers to a number of vertices by their indices.
        * This array is always present in a mesh, its size is given
        * in mNumFaces. If the #AI_SCENE_FLAGS_NON_VERBOSE_FORMAT
        * is NOT set each face references an unique set of vertices.
        */
    // C_STRUCT aiFace* mFaces;
    mFaces: *mut Face,

    /** The number of bones this mesh contains.
        * Can be 0, in which case the mBones array is NULL.
        */
    // unsigned int mNumBones;
    mNumBones: c_uint,

    /** The bones of this mesh.
        * A bone consists of a name by which it can be found in the
        * frame hierarchy and a set of vertex weights.
        */
    // C_STRUCT aiBone** mBones;
    mBones: *mut*mut Bone,

    /** The material used by this mesh.
        * A mesh does use only a single material. If an imported model uses
        * multiple materials, the import splits up the mesh. Use this value
        * as index into the scene's material list.
        */
    // unsigned int mMaterialIndex;
    mMaterialIndex: c_uint,

    /** Name of the mesh. Meshes can be named, but this is not a
        *  requirement and leaving this field empty is totally fine.
        *  There are mainly three uses for mesh names:
        *   - some formats name nodes and meshes independently.
        *   - importers tend to split meshes up to meet the
        *      one-material-per-mesh requirement. Assigning
        *      the same (dummy) name to each of the result meshes
        *      aids the caller at recovering the original mesh
        *      partitioning.
        *   - Vertex animations refer to meshes by their names.
        **/
    // C_STRUCT aiString mName;
    mName: aiString,


    /** NOT CURRENTLY IN USE. The number of attachment meshes */
    // unsigned int mNumAnimMeshes;
    mNumAnimMeshes: c_uint,

    /** NOT CURRENTLY IN USE. Attachment meshes for this mesh, for vertex-based animation.
        *  Attachment meshes carry replacement data for some of the
        *  mesh'es vertex components (usually positions, normals). */
    // C_STRUCT aiAnimMesh** mAnimMeshes;
    mAnimMeshes: *mut*mut AnimMesh,
}
