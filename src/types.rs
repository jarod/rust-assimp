//! Defines basic data types and primitives used by assimp.

use std::str;
use std::fmt;
use libc::{c_float, size_t, c_uchar, c_uint};
use std::num::Float;

use vecmath as m;

use ffi;

/// Maximum dimension for strings, ASSIMP strings are zero terminated.
const MAXLEN : uint = 1024u;

/// Boolean type used by assimp.
#[doc(hidden)]
#[deriving(Clone, PartialEq, Eq, Show)]
#[repr(C)]
pub enum AiBool {
    /// Represents false
    AiFalse = 0x0,
    /// Represents true
    AiTrue = 0x1,
}

impl AiBool {
    /// Creates a new `AiBool` from the builtin `bool`.
    pub fn new(val: bool) -> AiBool {
        match val {
            true => AiBool::AiTrue,
            false => AiBool::AiFalse,
        }
    }
}

///	Standard return type for some library functions.
#[repr(C)]
pub enum Return {
    /// Indicates that a function was successful
    Success = 0x0,

    /// Indicates that a function failed
    Failure = -0x1,

    /// Indicates that not enough memory was availabe to perform the requested
    /// operation
    OutOfMemory = -0x3,
}

/// Represents a plane in a three-dimensional, euclidean space.
///
/// The components are the coefficients in the equation
/// `ax + by + cz + d = 0`.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Plane {
    /// x coefficient in the plane equation
    pub a: c_float,
    /// y coefficient in the plane equation
    pub b: c_float,
    /// z coefficient in the plane equation
    pub c: c_float,
    /// constant in the plane equation
    pub d: c_float,
}

/// Represents a ray.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Ray {
    /// Position of the ray
    pub pos: Vector3D,
    /// Direction of the ray
    pub dir: Vector3D,
}

/// Represents a color in Red-Green-Blue space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Color3D {
    /// Red component
    pub r: c_float,
    /// Green component
    pub g: c_float,
    /// Blue component
    pub b: c_float,
}

/// Represents a color in Red-Green-Blue-Alpha space.
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Color4D {
    /// Red component
    pub r: c_float,
    /// Green component
    pub g: c_float,
    /// Blue component
    pub b: c_float,
    /// Alpha component
    pub a: c_float,
}

/// Stores the memory requirements for different components.
///
/// All sizes are in bytes. Returned by Scene::get_memory_info()
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
/// The character set of an `AiString` is explicitly defined to be UTF-8. This
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
    /// Create a new empty string
    pub fn new() -> AiString {
        AiString {
            length: 0,
            data: [0u8, ..MAXLEN],
        }
    }

    /// Get a `str` representation of this `AiString`
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.data.slice_to((self.length) as uint))
    }

    /// Get a `String` representation of this `AiString`
    pub fn into_string(&self) -> Option<String> {
        match String::from_utf8((self.data.slice_to(self.length as uint))
                                .to_vec()) {
            Err(_) => None,
            Ok(s) => Some(s),
        }
    }
}

impl fmt::Show for AiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            Ok(s) => s.fmt(f),
            _    => "".fmt(f),
        }
    }
}

impl PartialEq for AiString {
    fn eq(&self, other: &AiString) -> bool {
        if self.length != other.length {
            return false
        }

        for i in range(0u, self.length as uint) {
            if self.data[i] != other.data[i] {
                return false
            }
        }
        return true
    }
}

/// Represents a vector in 2 dimensional space.
#[deriving(Clone, PartialEq, Show, Copy)]
#[repr(C, packed)]
pub struct Vector2D {
    /// x component
    pub x: c_float,
    /// y component
    pub y: c_float,
}

impl Vector2D {
    /// Create an array representation of the vector
    pub fn to_array(&self) -> [c_float, ..2] {
        [self.x, self.y]
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x +
        self.y * other.y
    }

    /// Calculate the norm of the vector
    #[inline]
    pub fn norm(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Calculate the norm of the vector
    #[inline]
    pub fn rnorm(&self) -> f32 {
        self.dot(self).rsqrt()
    }

    /// Normalize the vector
    #[inline]
    pub fn normalize(&mut self) -> Vector2D {
        (*self) * self.rnorm()
    }
}

impl Add<Vector2D, Vector2D> for Vector2D {
    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector2D, Vector2D> for Vector2D {
    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32, Vector2D> for Vector2D {
    fn mul(self, rhs: f32) -> Vector2D {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2D, Vector2D> for f32 {
    fn mul(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<f32, Vector2D> for Vector2D {
    fn div(self, rhs: f32) -> Vector2D {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Represents a vector in 3 dimensional space.
#[deriving(Clone, PartialEq, Show, Copy)]
#[repr(C, packed)]
pub struct Vector3D {
    /// x component
    pub x: c_float,
    /// y component
    pub y: c_float,
    /// z component
    pub z: c_float,
}

impl Vector3D {
    /// Create an array representation of the vector
    pub fn to_array(&self) -> [c_float, ..3] {
        [self.x, self.y, self.z]
    }

    /// Create a translation matrix from this vector
    pub fn translation_matrix(&self) -> Matrix4x4 {
        Matrix4x4 {
            a1: 1.0, a2: 0.0, a3: 0.0, a4: self.x,
            b1: 0.0, b2: 1.0, b3: 0.0, b4: self.y,
            c1: 0.0, c2: 0.0, c3: 1.0, c4: self.z,
            d1: 0.0, d2: 0.0, d3: 0.0, d4: 1.0,
        }
    }

    /// Create a scaling matrix from this vector
    pub fn scaling_matrix(&self) -> Matrix4x4 {
        Matrix4x4 {
            a1: self.x, a2: 0.0,    a3: 0.0,    a4: 0.0,
            b1: 0.0,    b2: self.y, b3: 0.0,    b4: 0.0,
            c1: 0.0,    c2: 0.0,    c3: self.z, c4: 0.0,
            d1: 0.0,    d2: 0.0,    d3: 0.0,    d4: 1.0,
        }
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(&self, other: &Vector3D) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /// Calculate the norm of the vector
    #[inline]
    pub fn norm(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Calculate the norm of the vector
    #[inline]
    pub fn rnorm(&self) -> f32 {
        self.dot(self).rsqrt()
    }

    /// Normalize the vector
    #[inline]
    pub fn normalize(&mut self) -> Vector3D {
        (*self) * self.rnorm()
    }
}

impl Add<Vector3D, Vector3D> for Vector3D {
    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector3D, Vector3D> for Vector3D {
    fn sub(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32, Vector3D> for Vector3D {
    fn mul(self, rhs: f32) -> Vector3D {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3D, Vector3D> for f32 {
    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32, Vector3D> for Vector3D {
    fn div(self, rhs: f32) -> Vector3D {
        Vector3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Represents a quaternion.
#[allow(missing_docs)]
#[deriving(Clone, PartialEq, Show, Copy)]
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

impl Quaternion {
    /// Creates a rotation quaternion from the given matrix
    pub fn from_matrix(mat: &Matrix3x3) -> Quaternion {
        let mut quat: Quaternion = Quaternion::zero();
        unsafe {
            ffi::aiCreateQuaternionFromMatrix(&mut quat, mat);
        }
        quat
    }

    /// Create an array representation of the vector
    pub fn to_array(&self) -> [c_float, ..4] {
        [self.w, self.x, self.y, self.z]
    }

    /// Create a rotation matrix from this quaternion
    pub fn rotation_matrix(&self) -> Matrix4x4 {
        let norm = self.dot(self);
        let s = if norm < 1e-6 {
            0.0
        } else {
            2.0 / norm
        };
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let wx = s*w*x; let wy = s*w*y; let wz = s*w*z;
        let xx = s*x*x; let xy = s*x*y; let xz = s*x*z;
        let yy = s*y*y; let yz = s*y*z; let zz = s*z*z;
        Matrix4x4 {
            a1:  1. - (yy + zz), a2:      (xy - wz), a3:      (xz + wy), a4: 0.0,
            b1:       (xy + wz), b2: 1. - (xx + zz), b3:      (yz - wx), b4: 0.0,
            c1:       (xz - wy), c2:      (yz + wx), c3: 1. - (xx + yy), c4: 0.0,
            d1:             0.0, d2:            0.0, d3:            0.0, d4: 1.0,
        }
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.w * other.w +
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /// Calculate the norm of the quaternion
    #[inline]
    pub fn norm(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Calculate the norm of the quaternion
    #[inline]
    pub fn rnorm(&self) -> f32 {
        self.dot(self).rsqrt()
    }

    /// Normalize the quaternion
    #[inline]
    pub fn normalize(&mut self) -> Quaternion {
        (*self) * self.rnorm()
    }
}

impl Add<Quaternion, Quaternion> for Quaternion {
    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Quaternion, Quaternion> for Quaternion {
    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32, Quaternion> for Quaternion {
    fn mul(self, rhs: f32) -> Quaternion {
        Quaternion {
            w: self.w * rhs,
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Quaternion, Quaternion> for f32 {
    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self * rhs.w,
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32, Quaternion> for Quaternion {
    fn div(self, rhs: f32) -> Quaternion {
        Quaternion {
            w: self.w / rhs,
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Represents a 3x3 matrix.
#[allow(missing_docs)]
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Matrix3x3 {
    pub a1: c_float, pub a2: c_float, pub a3: c_float,
    pub b1: c_float, pub b2: c_float, pub b3: c_float,
    pub c1: c_float, pub c2: c_float, pub c3: c_float,
}

impl Matrix3x3 {
    /// Create a 3x3 identity matrix
    pub fn identity() -> Matrix3x3 {
        Matrix3x3 {
            a1:  1.0, a2: 0.0, a3: 0.0,
            b1:  0.0, b2: 1.0, b3: 0.0,
            c1:  0.0, c2: 0.0, c3: 1.0,
        }
    }

    /// Compute the inverse of a 3x3 matrix
    pub fn inverse(self) -> Matrix3x3 {
        let inv = m::mat3_inv([
                    [self.a1, self.a2, self.a3],
                    [self.b1, self.b2, self.b3],
                    [self.c1, self.c2, self.c3],
                  ]);
        Matrix3x3 {
            a1:  inv[0][0], a2: inv[0][1], a3: inv[0][2],
            b1:  inv[1][0], b2: inv[1][1], b3: inv[1][2],
            c1:  inv[2][0], c2: inv[2][1], c3: inv[2][2],
        }
    }

    /// Returns the transpose of this matrix
    pub fn transpose(&self) -> Matrix3x3 {
        let mut copy = self.clone();
        unsafe {
            ffi::aiTransposeMatrix3(&mut copy)
        }
        copy
    }

}

impl Mul<Matrix3x3, Matrix3x3> for Matrix3x3 {
    fn mul(self, rhs: Matrix3x3) -> Matrix3x3 {
        let mut result = self.clone();
        unsafe {
            ffi::aiMultiplyMatrix3(&mut result, &rhs)
        }
        result
    }
}

/// Represents a 4x4 matrix.
#[allow(missing_docs)]
#[deriving(Clone, PartialEq, Show)]
#[repr(C, packed)]
pub struct Matrix4x4 {
    pub a1: c_float, pub a2: c_float, pub a3: c_float, pub a4: c_float,
    pub b1: c_float, pub b2: c_float, pub b3: c_float, pub b4: c_float,
    pub c1: c_float, pub c2: c_float, pub c3: c_float, pub c4: c_float,
    pub d1: c_float, pub d2: c_float, pub d3: c_float, pub d4: c_float,
}

impl Matrix4x4 {
    /// Create a 4x4 identity matrix
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            a1:  1.0, a2: 0.0, a3: 0.0, a4: 0.0,
            b1:  0.0, b2: 1.0, b3: 0.0, b4: 0.0,
            c1:  0.0, c2: 0.0, c3: 1.0, c4: 0.0,
            d1:  0.0, d2: 0.0, d3: 0.0, d4: 1.0,
        }
    }

    /// Returns a slice equivalent to this matrix in row-major format
    pub fn to_array(&self) -> [[f32, ..4], ..4] {
        [
            [self.a1, self.a2, self.a3, self.a4,],
            [self.b1, self.b2, self.b3, self.b4,],
            [self.c1, self.c2, self.c3, self.c4,],
            [self.d1, self.d2, self.d3, self.d4,],
        ]
    }

    /// Returns the transpose of this matrix
    pub fn transpose(&self) -> Matrix4x4 {
        let mut copy = self.clone();
        unsafe {
            ffi::aiTransposeMatrix4(&mut copy)
        }
        copy
    }

    /// Compute the inverse of a 4x4 matrix
    pub fn inverse(&self) -> Matrix4x4 {
        let inv = m::mat4_inv(self.to_array());
        Matrix4x4 {
            a1:  inv[0][0], a2: inv[0][1], a3: inv[0][2], a4: inv[0][3],
            b1:  inv[1][0], b2: inv[1][1], b3: inv[1][2], b4: inv[1][3],
            c1:  inv[2][0], c2: inv[2][1], c3: inv[2][2], c4: inv[2][3],
            d1:  inv[3][0], d2: inv[3][1], d3: inv[3][2], d4: inv[3][3],
        }
    }
}

impl Mul<Matrix4x4, Matrix4x4> for Matrix4x4 {
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        let mut result = self.clone();
        unsafe {
            ffi::aiMultiplyMatrix4(&mut result, &rhs)
        }
        result
    }
}

// #[cfg(test)]
// mod test {
//     use super::Matrix4x4;
//     #[test]
//     fn test_inv() {
//         Matrix4x4::identity() * 
//     }
// }
