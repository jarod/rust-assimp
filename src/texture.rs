//! Defines texture helper structures for the library
//!
//! Used for file formats which embed their textures into the model file.
//! Supported are both normal textures, which are stored as uncompressed
//! pixels, and "compressed" textures, which are stored in a file format
//! such as PNG or TGA.

use libc::{c_uchar, c_char, c_uint};

use util::{ptr_to_slice};

// /// @def AI_MAKE_EMBEDDED_TEXNAME
// ///
// /// Used to build the reserved path name used by the material system to
// /// reference textures that are embedded into their corresponding
// /// model files. The parameter specifies the index of the texture
// /// (zero-based, in the aiScene::mTextures array)
// ///
// #if (!defined AI_MAKE_EMBEDDED_TEXNAME)
// #	define AI_MAKE_EMBEDDED_TEXNAME(_n_) "*" # _n_
// #endif


/// Helper structure to represent a texel in a ARGB8888 format
#[repr(C, packed)]
pub struct Texel {
    /// blue
    b: c_uchar,
    /// green
    g: c_uchar,
    /// red
    r: c_uchar,
    /// alpha
    a: c_uchar,
}

///  Helper structure to describe an embedded texture
///
/// Normally textures are contained in external files but some file formats
/// embed them directly in the model file. There are two types of embedded
/// textures:
///
/// 1. Uncompressed textures. The color data is given in an uncompressed
///    format.
/// 2. Compressed textures stored in a file format like png or jpg. The raw
///    file bytes are given so the application must utilize an image decoder
///    to get access to the actual color data.
#[repr(C)] // not packed
pub struct Texture {
    /// Width of the texture, in pixels
    ///
    /// If height is zero the texture is compressed in a format
    /// like JPEG. In this case mWidth specifies the size of the
    /// memory area pcData is pointing to, in bytes.
    pub width: c_uint,

    /// Height of the texture, in pixels
    ///
    /// If this value is zero, pcData points to an compressed texture
    /// in any format (e.g. JPEG).
    pub height: c_uint,

    /// A hint from the loader to make it easier for applications
    /// to determine the type of embedded compressed textures.
    ///
    /// If height != 0 this member is undefined. Otherwise it
    /// is set set to '\\0\\0\\0\\0' if the loader has no additional
    /// information about the texture file format used OR the
    /// file extension of the format without a trailing dot. If there
    /// are multiple file extensions for a format, the shortest
    /// extension is chosen (JPEG maps to 'jpg', not to 'jpeg').
    /// E.g. 'dds\\0', 'pcx\\0', 'jpg\\0'.  All characters are lower-case.
    /// The fourth character will always be '\\0'.
    pub format_hint: [c_char, ..4],

    /// Data of the texture.
    ///
    /// Points to an array of `width * height` Texel's.
    /// The format of the texture data is always ARGB8888 to
    /// make the implementation for user of the library as easy
    /// as possible. If mHeight = 0 this is a pointer to a memory
    /// buffer of size mWidth containing the compressed texture
    /// data. Good luck, have fun!
    ///
    pc_data: *mut Texel
}

/// Texture data can be encoded or decoded
pub enum TextureData<'a> {
    /// Encoded texture data
    Encoded {
        /// The length of the texture data in bytes
        pub len: u32,
        /// The encoded texture data
        pub data: &'a [u8],
    },
    /// Decoded texture data
    Decoded {
        /// The width of the texture data in texels
        pub width: u32,
        /// The height of the texture data in texels
        pub height: u32,
        /// A linear array store the texels in the texture
        pub data: &'a [Texel]
    },
}

impl Texture {
    /// Get the embeded texture data
    pub fn get_texture_data(&self) -> TextureData {
        if self.height == 0 {
            let data = unsafe {
                ptr_to_slice(self.pc_data as *mut u8, self.width as uint)
            };
            Encoded {
                len: self.width,
                data: data,
            }
        } else {
            let data = unsafe {
                ptr_to_slice(self.pc_data, (self.width * self.height) as uint)
            };
            Decoded {
                width: self.width,
                height: self.height,
                data: data,
            }
        }
    }
}
