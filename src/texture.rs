//! Defines texture helper structures for the library
//!
//! Used for file formats which embed their textures into the model file.
//! Supported are both normal textures, which are stored as uncompressed
//! pixels, and "compressed" textures, which are stored in a file format
//! such as PNG or TGA.

use libc::{c_uchar, c_char, c_uint};

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
    b: c_uchar,
    g: c_uchar,
    r: c_uchar,
    a: c_uchar,
}

    /** Helper structure to describe an embedded texture
     * 
     * Normally textures are contained in external files but some file formats embed
     * them directly in the model file. There are two types of embedded textures: 
     * 1. Uncompressed textures. The color data is given in an uncompressed format. 
     * 2. Compressed textures stored in a file format like png or jpg. The raw file 
     * bytes are given so the application must utilize an image decoder (e.g. DevIL) to
     * get access to the actual color data.
     */
#[repr(C)] // not packed
pub struct Texture {
    /** Width of the texture, in pixels
        *
        * If mHeight is zero the texture is compressed in a format
        * like JPEG. In this case mWidth specifies the size of the
        * memory area pcData is pointing to, in bytes.
        */
    /// Width of the texture, in pixels
    // unsigned int mWidth;
    mWidth: c_uint,

    /** Height of the texture, in pixels
        *
        * If this value is zero, pcData points to an compressed texture
        * in any format (e.g. JPEG).
        */
    /// Height of the texture, in pixels
    // unsigned int mHeight;
    mHeight: c_uint,

    /** A hint from the loader to make it easier for applications
        *  to determine the type of embedded compressed textures.
        *
        * If mHeight != 0 this member is undefined. Otherwise it
        * is set set to '\\0\\0\\0\\0' if the loader has no additional
        * information about the texture file format used OR the
        * file extension of the format without a trailing dot. If there 
        * are multiple file extensions for a format, the shortest 
        * extension is chosen (JPEG maps to 'jpg', not to 'jpeg').
        * E.g. 'dds\\0', 'pcx\\0', 'jpg\\0'.  All characters are lower-case.
        * The fourth character will always be '\\0'.
        */
    // char achFormatHint[4];
    achFormatHint: [c_char, ..4],

    /** Data of the texture.
        *
        * Points to an array of mWidth * mHeight aiTexel's.
        * The format of the texture data is always ARGB8888 to
        * make the implementation for user of the library as easy
        * as possible. If mHeight = 0 this is a pointer to a memory
        * buffer of size mWidth containing the compressed texture
        * data. Good luck, have fun!
        */
    // C_STRUCT aiTexel* pcData;
    pcData: *mut Texel
}
