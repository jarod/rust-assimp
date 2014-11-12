/** @file  aiVersion.h
 *  @brief Functions to query the version of the Assimp runtime, check
 *    compile flags, ...
 */
use libc::{c_char, c_uint};
use std::c_str::CString;

// /// Flags for cheching how assimp was compiled
// enum AssimpCFlags {
//     /// Assimp was compiled as a shared object (Windows: DLL)
//     ASSIMP_CFLAGS_SHARED = 0x1,
//     /// Assimp was compiled against STLport
//     ASSIMP_CFLAGS_STLPORT = 0x2,
//     /// Assimp was compiled as a debug build
//     ASSIMP_CFLAGS_DEBUG = 0x4,
//     /// Assimp was compiled with ASSIMP_BUILD_BOOST_WORKAROUND defined
//     ASSIMP_CFLAGS_NOBOOST = 0x8,
//     /// Assimp was compiled with ASSIMP_BUILD_SINGLETHREADED defined
//     ASSIMP_CFLAGS_SINGLETHREADED = 0x10,
// }

#[link(name = "assimp")]
extern {
    // ASSIMP_API const char*  aiGetLegalString  (void);
    fn aiGetLegalString() -> *const c_char;

    // ASSIMP_API unsigned int aiGetVersionMinor (void);
    fn aiGetVersionMinor() -> c_uint;

    // ASSIMP_API unsigned int aiGetVersionMajor (void);
    fn aiGetVersionMajor() -> c_uint;

    // ASSIMP_API unsigned int aiGetVersionRevision (void);
    // fn aiGetVersionRevision() -> c_uint;

    // ASSIMP_API unsigned int aiGetCompileFlags (void);
    // fn aiGetCompileFlags() -> c_uint;
}

/// Get the major and minor version number as tuple (major, minor)
pub fn get_version() -> (uint, uint) {
    let major = unsafe {
        aiGetVersionMajor() as uint
    };
    let minor : uint = unsafe {
        aiGetVersionMinor() as uint
    };
    (major, minor)
}

/// Get the major and minor version number as tuple (major, minor)
pub fn get_legal_string() -> CString {
    unsafe {
        CString::new(aiGetLegalString(), false)
    }
}
