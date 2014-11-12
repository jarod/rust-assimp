//! Basic data types and primitives, such as vectors or colors.

use std::str;
use libc::{c_float, size_t, c_uchar};

/// Maximum dimension for strings, ASSIMP strings are zero terminated.
const MAXLEN : uint = 1024u;

/// Represents a plane in a three-dimensional, euclidean space
#[repr(C, packed)]
pub struct Plane {
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
}

/// Represents a ray
#[repr(C, packed)]
pub struct Ray {
    /// Position and direction of the ray
    pos: Vector3D,
    dir: Vector3D,
}

/// Represents a color in Red-Green-Blue space. 
#[repr(C, packed)]
pub struct Color3D
{
    r: c_float,
    g: c_float,
    b: c_float,
}

/** Represents an UTF-8 string, zero byte terminated.
*
*  The character set of an aiString is explicitly defined to be UTF-8. This Unicode
*  transformation was chosen in the belief that most strings in 3d files are limited
*  to ASCII, thus the character set needed to be strictly ASCII compatible.
*  
*  Most text file loaders provide proper Unicode input file handling, special unicode
*  characters are correctly transcoded to UTF8 and are kept throughout the libraries'
*  import pipeline. 
*
*  For most applications, it will be absolutely sufficient to interpret the
*  aiString as ASCII data and work with it as one would work with a plain char*. 
*  Windows users in need of proper support for i.e asian characters can use the
*  #MultiByteToWideChar(), #WideCharToMultiByte() WinAPI functionality to convert the
*  UTF-8 strings to their working character set (i.e. MBCS, WideChar).
*
*  We use this representation instead of std::string to be C-compatible. The 
*  (binary) length of such a string is limited to MAXLEN characters (including the
*  the terminating zero).
*/

/// Represents an UTF-8 string, zero byte terminated.
#[repr(C, packed)]
pub struct AiString {
/** Binary length of the string excluding the terminal 0. This is NOT the 
    *  logical length of strings containing UTF-8 multibyte sequences! It's
    *  the number of bytes from the beginning of the string to its end.*/
    length: size_t,

    /** String buffer. Size limit is MAXLEN */
    data: [c_uchar, ..MAXLEN],
}

impl AiString {
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self.data.slice_to(self.length as uint))
    }
}

// //   /**	Standard return type for some library functions.//{{{
// //    * Rarely used, and if, mostly in the C API.
// //    */
// //   enum aiReturn
// //   {
// //     /** Indicates that a function was successful */
// //     aiReturn_SUCCESS = 0x0,

// //     /** Indicates that a function failed */
// //     aiReturn_FAILURE = -0x1,

// //     /** Indicates that not enough memory was available
// //      * to perform the requested operation 
// //      */
// //     aiReturn_OUTOFMEMORY = -0x3,

// //     /** @cond never 
// //      *  Force 32-bit size enum
// //      */
// //     _AI_ENFORCE_ENUM_SIZE = 0x7fffffff 
// //   };  // !enum aiReturn

// //   /** Seek origins (for the virtual file system API).
// //    *  Much cooler than using SEEK_SET, SEEK_CUR or SEEK_END.
// //    */
// //   enum aiOrigin
// //   {
// //     /** Beginning of the file */
// //     aiOrigin_SET = 0x0,	

// //     /** Current position of the file pointer */
// //     aiOrigin_CUR = 0x1,		

// //     /** End of the file, offsets must be negative */
// //     aiOrigin_END = 0x2,

// //     /**  @cond never 
// //      *   Force 32-bit size enum 
// //      */
// //     _AI_ORIGIN_ENFORCE_ENUM_SIZE = 0x7fffffff 
// //   }; // !enum aiOrigin

// //   /** @brief Enumerates predefined log streaming destinations. 
// //    *  Logging to these streams can be enabled with a single call to 
// //    *   #LogStream::createDefaultStream or #aiAttachPredefinedLogStream(),
// //    *   respectively.
// //    */
// //   enum aiDefaultLogStream	
// //   {
// //     /** Stream the log to a file */
// //     aiDefaultLogStream_FILE = 0x1,

// //     /** Stream the log to std::cout */
// //     aiDefaultLogStream_STDOUT = 0x2,

// //     /** Stream the log to std::cerr */
// //     aiDefaultLogStream_STDERR = 0x4,

// //     /** MSVC only: Stream the log the the debugger
// //      * (this relies on OutputDebugString from the Win32 SDK)
// //      */
// //     aiDefaultLogStream_DEBUGGER = 0x8,

// //     /** @cond never 
// //      *  Force 32-bit size enum 
// //      */
// //     _AI_DLS_ENFORCE_ENUM_SIZE = 0x7fffffff 
// //   }; // !enum aiDefaultLogStream

// //   /** Stores the memory requirements for different components (e.g. meshes, materials,
// //    *  animations) of an import. All sizes are in bytes.
// //    *  @see Importer::GetMemoryRequirements()
// //    */
// //   struct aiMemoryInfo
// //   {

// //     /** Storage allocated for texture data */
// //     unsigned int textures;

// //     /** Storage allocated for material data  */
// //     unsigned int materials;

// //     /** Storage allocated for mesh data */
// //     unsigned int meshes;

// //     /** Storage allocated for node data */
// //     unsigned int nodes;

// //     /** Storage allocated for animation data */
// //     unsigned int animations;

// //     /** Storage allocated for camera data */
// //     unsigned int cameras;

// //     /** Storage allocated for light data */
// //     unsigned int lights;

// //     /** Total storage allocated for the full import. */
// //     unsigned int total;
// //   }; // !struct aiMemoryInfo //}}}

#[repr(C, packed)]
pub struct Vector2D {
    x: c_float,
    y: c_float,
}

#[repr(C, packed)]
pub struct Vector3D {
    x: c_float,
    y: c_float,
    z: c_float,
}

#[repr(C, packed)]
pub struct Color4D {
    r: c_float,
    g: c_float,
    b: c_float,
    a: c_float,
}

#[repr(C, packed)]
pub struct Quaternion {
    w: c_float,
    x: c_float,
    y: c_float,
    z: c_float,
}

#[repr(C, packed)]
pub struct Matrix3x3 {
    a1: c_float, a2: c_float, a3: c_float,
    b1: c_float, b2: c_float, b3: c_float,
    c1: c_float, c2: c_float, c3: c_float,
}

#[repr(C, packed)]
pub struct Matrix4x4 {
    a1: c_float, a2: c_float, a3: c_float, a4: c_float,
    b1: c_float, b2: c_float, b3: c_float, b4: c_float,
    c1: c_float, c2: c_float, c3: c_float, c4: c_float,
    d1: c_float, d2: c_float, d3: c_float, d4: c_float,
}
