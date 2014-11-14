//!  Functions to query the version of the Assimp runtime, check compile flags, ...

use libc::{c_char, c_uint};
use types::{AiString, AiBool, AiTrue};
use std::c_str::CString;

/// Flags for cheching how assimp was compiled
enum AssimpCFlags {
    /// Assimp was compiled as a shared object (Windows: DLL)
    ASSIMP_CFLAGS_SHARED = 0x1,

    /// Assimp was compiled against STLport
    ASSIMP_CFLAGS_STLPORT = 0x2,

    /// Assimp was compiled as a debug build
    ASSIMP_CFLAGS_DEBUG = 0x4,

    /// Assimp was compiled with ASSIMP_BUILD_BOOST_WORKAROUND defined
    ASSIMP_CFLAGS_NOBOOST = 0x8,

    /// Assimp was compiled with ASSIMP_BUILD_SINGLETHREADED defined
    ASSIMP_CFLAGS_SINGLETHREADED = 0x10,
}

#[link(name = "assimp")]
extern {
    fn aiGetLegalString() -> *const c_char;

    fn aiGetVersionMinor() -> c_uint;

    fn aiGetVersionMajor() -> c_uint;

    fn aiGetVersionRevision() -> c_uint;

    fn aiGetCompileFlags() -> c_uint;

    fn aiGetExtensionList(out: *mut AiString);

    fn aiIsExtensionSupported(ext: *const c_char) -> AiBool;
}

/// Get the assimp version number as a tuple (major, minor, revision)
pub fn get_version() -> (uint, uint, uint) {
    let major = unsafe {
        aiGetVersionMajor() as uint
    };
    let minor = unsafe {
        aiGetVersionMinor() as uint
    };
    let rev = unsafe {
        aiGetVersionRevision() as uint
    };
    (major, minor, rev)
}

/// Get the a string containg the assimp licene
pub fn get_legal_string() -> String {
    unsafe {
        CString::new(aiGetLegalString(), false).to_string()
    }
}

/// Get a list supported file formats in the form "*.3ds;*.obj;*.dae".
///
/// If a file extension is contained in the list this does, of course, not
/// mean that ASSIMP is able to load all files with this extension.
pub fn get_supported_exts() -> String {
    let mut exts = AiString::new();
    unsafe {
        aiGetExtensionList(&mut exts);
    }
    // Don't expect this to fail
    exts.as_str().unwrap().into_string()
}

/// Returns whether a given file extension is supported by assimp
///
/// # Parameters
///
/// * `extension` Extension for which the function queries support for.
///   Must include a leading dot '.'. Example: ".3ds", ".md3"
pub fn is_supported(ext: &str) -> bool {
    unsafe {
        ext.with_c_str(|s| aiIsExtensionSupported(s)) == AiTrue
    }
}

#[cfg(test)]
mod test {
    use info;

    #[test]
    fn test_version() {
        // Hello world test
        let (major, minor, rev) = info::get_version();
        println!("({}, {}, {})", major, minor, rev);

        // Get supported formats
        println!("{}\n", info::get_supported_exts());

        // Get licencing
        println!("{}", info::get_legal_string());

        // Check is supported
        println!("support dae : {}", info::is_supported(".dae"));
        println!("support md3 : {}", info::is_supported(".md3"));
        println!("support mad : {}", info::is_supported(".mad"));
    }
}
