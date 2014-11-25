
use libc::{c_uint, c_float};
use types;
use material;

/// brief Helper function to get all values pertaining to a particular
///   texture slot from a material structure.
///
///   This function is provided just for convenience. You could also read the
///   texture by parsing all of its properties manually. This function bundles
///   all of them in a huge function monster.
///
///   @param[in] mat Pointer to the input material. May not be NULL
///   @param[in] type Specifies the texture stack to read from (e.g. diffuse,
///      specular, height map ...).
///   @param[in] index Index of the texture. The function fails if the
///      requested index is not available for this texture type.
///      #aiGetMaterialTextureCount() can be used to determine the number of
///      textures in a particular texture stack.
///   @param[out] path Receives the output path
///       This parameter must be non-null.
///   @param mapping The texture mapping mode to be used.
///       Pass NULL if you're not interested in this information.
///   @param[out] uvindex For UV-mapped textures: receives the index of the UV
///       source channel. Unmodified otherwise.
///       Pass NULL if you're not interested in this information.
///   @param[out] blend Receives the blend factor for the texture
///       Pass NULL if you're not interested in this information.
///   @param[out] op Receives the texture blend operation to be perform between
///        this texture and the previous texture.
///       Pass NULL if you're not interested in this information.
///   @param[out] mapmode Receives the mapping modes to be used for the texture.
///       Pass NULL if you're not interested in this information. Otherwise,
///       pass a pointer to an array of two aiTextureMapMode's (one for each
///       axis, UV order).
///   @return AI_SUCCESS on success, otherwise something else. Have fun.*/
// C_ENUM aiReturn aiGetMaterialTexture(const C_STRUCT aiMaterial* mat,
//                                      C_ENUM aiTextureType type,
//                                      unsigned int  index,
//                                      C_STRUCT AiString* path,
//                                      C_ENUM aiTextureMapping* mapping    /*= NULL*/,
//                                      unsigned int* uvindex               /*= NULL*/,
//                                      float* blend                        /*= NULL*/,
//                                      C_ENUM aiTextureOp* op              /*= NULL*/,
//                                      C_ENUM aiTextureMapMode* mapmode    /*= NULL*/,
//                                      unsigned int* flags                 /*= NULL*/);

extern {
    pub fn aiGetMaterialTexture(aiMaterial: *const material::Material,
                             aiTextureType: material::TextureType,
                             index: c_uint,
                             path: *mut types::AiString,
                             mapping: *mut material::TextureMapping /*= NULL*/,
                             uvindex: *mut c_uint            /*= NULL*/,
                             blend: *mut c_float                    /*= NULL*/,
                             op: *mut material::TextureOp           /*= NULL*/,
                             mapmode: *mut material::TextureMapMode /*= NULL*/,
                             flags: *mut c_uint              /*= NULL*/) -> types::Return;
}
