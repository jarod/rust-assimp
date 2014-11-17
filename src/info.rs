//! Defines functions to retrieve information about the version of assimp being used.

use std::c_str::CString;

use types::AiBool::{AiTrue};
use types::AiString;
use ffi;

/// Flags for checking how assimp was compiled
#[repr(C, u32)]
pub enum CompileFlags {
    /// Assimp was compiled as a shared object (Windows: DLL)
    Shared = 0x1,

    /// Assimp was compiled against STLport
    STLPort = 0x2,

    /// Assimp was compiled as a debug build
    Debug = 0x4,

    /// Assimp was compiled with `ASSIMP_BUILD_BOOST_WORKAROUND` defined
    NoBoost = 0x8,

    /// Assimp was compiled with `ASSIMP_BUILD_SINGLETHREADED` defined
    SingleThreaded = 0x10,
}

// It appears assimp doesn't expose a way to get this information using the
// c-api.

// /// #aiImporterFlags, aiImporterDesc implementation.//{{{

// /// Mixed set of flags for #aiImporterDesc, indicating some features
// /// common to many importers*/
// #[repr(C)]
// pub enum ImporterFlags {
//     /// Indicates that there is a textual encoding of the
//     /// file format; and that it is supported.
//     aiImporterFlags_SupportTextFlavour = 0x1,

//     /// Indicates that there is a binary encoding of the
//     /// file format; and that it is supported.
//     aiImporterFlags_SupportBinaryFlavour = 0x2,

//     /// Indicates that there is a compressed encoding of the
//     /// file format; and that it is supported.
//     aiImporterFlags_SupportCompressedFlavour = 0x4,

//     /// Indicates that the importer reads only a very particular subset of the
//     /// file format.
//     ///
//     /// This happens commonly for declarative or procedural formats which
//     /// cannot easily be mapped to #aiScene
//     aiImporterFlags_LimitedSupport = 0x8,

//     /// Indicates that the importer is highly experimental and should be used
//     /// with care.
//     ///
//     /// This only happens for trunk
//     /// (i.e. SVN) versions, experimental code is not included
//     /// in releases. */
//     aiImporterFlags_Experimental = 0x10,
// }
// /// Meta information about a particular importer.
// ///
// /// Importers need to fill this structure, but they can freely decide how
// /// talkative they are.  A common use case for loader meta info is a user
// /// interface in which the user can choose between various import/export file
// /// formats. Building such an UI by hand means a lot of maintenance as
// /// importers/exporters are added to Assimp, so it might be useful to have a
// /// common mechanism to query some rough importer characteristics.
// #[repr(C)]
// pub struct ImporterDesc {
//     /// Full name of the importer (i.e. Blender3D importer)
//     const char* mName;

//     /// Original author (left blank if unknown or whole assimp team)
//     const char* mAuthor;

//     /// Current maintainer, left blank if the author maintains
//     const char* mMaintainer;

//     /// Implementation comments, i.e. unimplemented features
//     const char* mComments;

//     /// Any combination of the #aiLoaderFlags enumerated values.
//     /// These flags indicate some characteristics common to many
//     /// importers.
//     unsigned int mFlags;

//     /// Minimum format version that can be loaded im major.minor format,
//     /// both are set to 0 if there is either no version scheme 
//     /// or if the loader doesn't care.
//     unsigned int mMinMajor;
//     unsigned int mMinMinor;

//     /// Maximum format version that can be loaded im major.minor format,
//     /// both are set to 0 if there is either no version scheme 
//     /// or if the loader doesn't care. Loaders that expect to be
//     /// forward-compatible to potential future format versions should 
//     /// indicate  zero, otherwise they should specify the current
//     /// maximum version.
//     unsigned int mMaxMajor;
//     unsigned int mMaxMinor;

//     /// List of file extensions this importer can handle.
//     ///
//     /// List entries are separated by space characters.
//     /// All entries are lower case without a leading dot (i.e.
//     /// "xml dae" would be a valid value. Note that multiple
//     /// importers may respond to the same file extension -
//     /// assimp calls all importers in the order in which they
//     /// are registered and each importer gets the opportunity
//     /// to load the file until one importer "claims" the file. Apart
//     /// from file extension checks, importers typically use
//     /// other methods to quickly reject files (i.e. magic
//     /// words) so this does not mean that common or generic
//     /// file extensions such as XML would be tediously slow.
//     const char* mFileExtensions;
// }//}}}

/// Get the version number of assimp as a tuple `(major, minor, revision)`
pub fn get_version() -> (uint, uint, uint) {
    let major = unsafe {
        ffi::aiGetVersionMajor() as uint
    };
    let minor = unsafe {
        ffi::aiGetVersionMinor() as uint
    };
    let rev = unsafe {
        ffi::aiGetVersionRevision() as uint
    };
    (major, minor, rev)
}

/// Get a string containg the assimp licene
pub fn get_legal_string() -> String {
    unsafe {
        CString::new(ffi::aiGetLegalString(), false).to_string()
    }
}

/// Get a list supported file formats in the form `*.3ds;*.obj;*.dae`.
///
/// If a file extension is contained in the list this does, of course, not
/// mean that assimp is able to load all files with this extension.
pub fn get_supported_import_exts() -> String {
    let mut exts = AiString::new();
    unsafe {
        ffi::aiGetExtensionList(&mut exts);
    }
    // Don't expect this to fail
    exts.as_str().unwrap().into_string()
}

/// Returns the set compile flags
pub fn get_compile_flags() -> u32 {
    unsafe { ffi::aiGetCompileFlags() }
}

/// Check if a given compile flag is set
pub fn is_flag_set(flag: CompileFlags) -> bool {
    unsafe { ( ffi::aiGetCompileFlags() & flag as u32 ) != 0 }
}

/// Returns whether a given file extension is supported by assimp
///
/// # Parameters
///
/// * `extension` Extension for which the function queries support for.
///   Must include a leading dot '.'. Example: '.3ds'
pub fn is_ext_supported(ext: &str) -> bool {
    unsafe {
        ext.with_c_str(|s| ffi::aiIsExtensionSupported(s)) == AiTrue
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
        println!("{}\n", info::get_supported_import_exts());

        // Get licencing
        println!("{}", info::get_legal_string());

        // Check is supported
        println!("support dae : {}", info::is_ext_supported(".dae"));
        println!("support md3 : {}", info::is_ext_supported(".md3"));
        println!("support mad : {}", info::is_ext_supported(".mad"));
    }
}
