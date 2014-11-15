//! Defines basic data types and primitives used by assimp.

use std::str;
use std::fmt;
use libc::{c_float, size_t, c_uchar, c_uint};

/// Maximum dimension for strings, ASSIMP strings are zero terminated.
const MAXLEN : uint = 1024u;

/// Boolean type used by assimp.
#[deriving(Clone, PartialEq, Eq, Show)]
#[repr(C)]
pub enum AiBool {
    AiFalse = 0x0,
    AiTrue = 0x1,
}

///	Standard return type for some library functions.
#[repr(C)]
pub enum Return {
    /// Indicates that a function was successful 
    Return_SUCCESS = 0x0,

    /// Indicates that a function failed 
    Return_FAILURE = -0x1,

    /// Indicates that not enough memory was availabe to perform the requested 
    /// operation
    Return_OUTOFMEMORY = -0x3,
}

/// Represents a plane in a three-dimensional, euclidean space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Plane {
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
}

/// Represents a ray.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Ray {
    /// Position and direction of the ray
    pos: Vector3D,
    dir: Vector3D,
}

/// Represents a color in Red-Green-Blue space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Color3D {
    r: c_float,
    g: c_float,
    b: c_float,
}

/// Stores the memory requirements for different components.
///
/// All sizes are in bytes. Returned by Importer::GetMemoryRequirements()
#[deriving(Clone, PartialEq, Eq, Show)]
#[repr(C)]
pub struct MemoryInfo {
    /// Storage allocated for texture data
    pub textures: c_uint,

    /// Storage allocated for material data
    pub materials: c_uint,

    /// Storage allocated for mesh data
    pub meshes: c_uint,

    /// Storage allocated for node data
    pub nodes: c_uint,

    /// Storage allocated for animation data
    pub animations: c_uint,

    /// Storage allocated for camera data
    pub cameras: c_uint,

    /// Storage allocated for light data
    pub lights: c_uint,

    /// Total storage allocated for the full import.
    pub total: c_uint,
}

/// Represents an UTF-8 string, zero byte terminated.
///
/// The character set of an AiString is explicitly defined to be UTF-8. This
/// Unicode transformation was chosen in the belief that most strings in 3d
/// files are limited to ASCII, thus the character set needed to be strictly
/// ASCII compatible.
///
/// Most text file loaders provide proper Unicode input file handling, special
/// unicode characters are correctly transcoded to UTF8 and are kept
/// throughout the libraries' import pipeline.
///
/// For most applications, it will be absolutely sufficient to interpret the
/// aiString as ASCII data and work with it as one would work with a plain
/// char*.
///
/// The (binary) length of such a string is limited to MAXLEN characters
/// (including the the terminating zero).
#[repr(C, packed)]
pub struct AiString {
    /// Binary length of the string excluding the terminal 0. This is NOT the
    /// logical length of strings containing UTF-8 multibyte sequences! It's
    /// the number of bytes from the beginning of the string to its end.
    length: size_t,

    /// String buffer. Size limit is MAXLEN
    data: [c_uchar, ..MAXLEN],
}

impl AiString {
    pub fn new() -> AiString {
        AiString {
            length: 0,
            data: [0u8, ..MAXLEN],
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self.data.slice_to((self.length) as uint))
    }
}

impl fmt::Show for AiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            None    => "".fmt(f),
            Some(s) => s.fmt(f),
        }
    }
}

/// Represents a vector in 2 dimensional space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Vector2D {
    pub x: c_float,
    pub y: c_float,
}

/// Represents a vector in 3 dimensional space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Vector3D {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

/// Represents a color in Red-Green-Blue-Alpha space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Color4D {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

/// Represents a quaternion.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Quaternion {
    pub w: c_float,
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl Quaternion {
    fn zero() -> Quaternion {
        Quaternion { w: 0.0, x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add<Quaternion, Quaternion> for Quaternion {
    fn add(&self, rhs: &Quaternion) -> Quaternion {
        Quaternion { w: self.w + rhs.w,
                     x: self.x + rhs.x,
                     y: self.y + rhs.y,
                     z: self.z + rhs.z,
        }
    }
}

impl Quaternion {
    fn from_matrix(mat: &Matrix3x3) -> Quaternion {
        let mut quat: Quaternion = Quaternion::zero();
        unsafe {
            aiCreateQuaternionFromMatrix(&mut quat, mat);
        }
        quat
    }
}

/// Represents a 3x3 matrix.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Matrix3x3 {
    a1: c_float, a2: c_float, a3: c_float,
    b1: c_float, b2: c_float, b3: c_float,
    c1: c_float, c2: c_float, c3: c_float,
}

/// Represents a 4x4 matrix.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Matrix4x4 {
    a1: c_float, a2: c_float, a3: c_float, a4: c_float,
    b1: c_float, b2: c_float, b3: c_float, b4: c_float,
    c1: c_float, c2: c_float, c3: c_float, c4: c_float,
    d1: c_float, d2: c_float, d3: c_float, d4: c_float,
}

//TODO implement these in rust
extern {
    /** Construct a quaternion from a 3x3 rotation matrix.
    *  @param quat Receives the output quaternion.
    *  @param mat Matrix to 'quaternionize'.
    *  @see aiQuaternion(const aiMatrix3x3& pRotMatrix)
    */
    // ASSIMP_API void aiCreateQuaternionFromMatrix(
    //     C_STRUCT aiQuaternion* quat,
    //     const C_STRUCT aiMatrix3x3* mat);
    fn aiCreateQuaternionFromMatrix(quat: *mut Quaternion, mat: *const Matrix3x3);

        // /** Decompose a transformation matrix into its rotational, translational and
        // *  scaling components.
        // *
        // * @param mat Matrix to decompose
        // * @param scaling Receives the scaling component
        // * @param rotation Receives the rotational component
        // * @param position Receives the translational component.
        // * @see aiMatrix4x4::Decompose (aiVector3D&, aiQuaternion&, aiVector3D&) const;
        // */
        // // ASSIMP_API void aiDecomposeMatrix(
        // //     const C_STRUCT aiMatrix4x4* mat,
        // //     C_STRUCT aiVector3D* scaling,
        // //     C_STRUCT aiQuaternion* rotation,
        // //     C_STRUCT aiVector3D* position);

    // /** Transpose a 4x4 matrix.
    // *  @param mat Pointer to the matrix to be transposed
    // */
    // // ASSIMP_API void aiTransposeMatrix4( C_STRUCT aiMatrix4x4* mat);

    // /** Transpose a 3x3 matrix.
    // *  @param mat Pointer to the matrix to be transposed
    // */
    // // ASSIMP_API void aiTransposeMatrix3( C_STRUCT aiMatrix3x3* mat);

    // /** Transform a vector by a 3x3 matrix
    // *  @param vec Vector to be transformed.
    // *  @param mat Matrix to transform the vector with.
    // */
    // // ASSIMP_API void aiTransformVecByMatrix3(
    // //     C_STRUCT aiVector3D* vec,
    // //     const C_STRUCT aiMatrix3x3* mat);

    // /** Transform a vector by a 4x4 matrix
    // *  @param vec Vector to be transformed.
    // *  @param mat Matrix to transform the vector with.
    // */
    // // ASSIMP_API void aiTransformVecByMatrix4(
    // //     C_STRUCT aiVector3D* vec,
    // //     const C_STRUCT aiMatrix4x4* mat);

    // /** Multiply two 4x4 matrices.
    // *  @param dst First factor, receives result.
    // *  @param src Matrix to be multiplied with 'dst'.
    // */
    // // ASSIMP_API void aiMultiplyMatrix4(
    // //     C_STRUCT aiMatrix4x4* dst,
    // //     const C_STRUCT aiMatrix4x4* src);

    // /** Multiply two 3x3 matrices.
    // *  @param dst First factor, receives result.
    // *  @param src Matrix to be multiplied with 'dst'.
    // */
    // // ASSIMP_API void aiMultiplyMatrix3(
    // //     C_STRUCT aiMatrix3x3* dst,
    // //     const C_STRUCT aiMatrix3x3* src);

    // /** Get a 3x3 identity matrix.
    // *  @param mat Matrix to receive its personal identity
    // */
    // // ASSIMP_API void aiIdentityMatrix3( C_STRUCT aiMatrix3x3* mat);

    // /** Get a 4x4 identity matrix.
    // *  @param mat Matrix to receive its personal identity
    // */
    // // ASSIMP_API void aiIdentityMatrix4( C_STRUCT aiMatrix4x4* mat);
}
