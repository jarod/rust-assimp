//! The assimp export interface

use libc::{c_char, size_t, c_uint, c_void};

use types::{AiString, Return};
// use ffi::rawscene::RawScene;
use scene::RawScene;
use fileio::{AiFileIO};

/// Describes an file format which Assimp can export to.
/// Use #aiGetExportFormatCount() to
///
/// learn how many export formats the current Assimp build supports and
/// #aiGetExportFormatDescription() to retrieve a description of an export
/// format option.
#[repr(C)]
struct ExportFormatDesc {
    /// a short string ID to uniquely identify the export format. Use this ID
    /// string to specify which file format you want to export to when calling
    /// #aiExportScene().  Example: "dae" or "obj"
    pub id: *const c_char,

    /// A short description of the file format to present to users. Useful if
    /// you want to allow the user to select an export format.
    pub description: *const c_char,

    /// Recommended file extension for the exported file in lower case.
    pub file_ext: *const c_char,
}

/// Describes a blob of exported scene data.
///
/// Use #aiExportSceneToBlob() to create a blob containing an exported scene.
/// The memory referred by this structure is owned by Assimp. Use
/// #aiReleaseExportedFile() to free its resources. Don't try to free the
/// memory on your side - it will crash for most build configurations due to
/// conflicting heaps.
///
/// Blobs can be nested - each blob may reference another blob, which may in
/// turn reference another blob and so on.  This is used when exporters write
/// more than one output file for a given #aiScene. See the remarks for
/// #aiExportDataBlob::name for more information.
#[repr(C)]
struct RawExportDataBlob {
    /// Size of the data in bytes
    pub size: size_t,

    /// The data.
    data: *const c_void,

    /// Name of the blob.
    ///
    /// An empty string always indicates the first (and primary) blob, which
    /// contains the actual file data.  Any other blobs are auxiliary files
    /// produced by exporters (i.e. material files). Existence of such files
    /// depends on the file format. Most formats don't split assets across
    /// multiple files.
    ///
    /// If used, blob names usually contain the file extension that should be
    /// used when writing the data to disc.
    pub name: AiString,

    /// Pointer to the next blob in the chain or NULL if there is none.
    next: *const RawExportDataBlob
}

struct ExportDataBlob {
    raw: *const RawExportDataBlob,
}

impl Drop for ExportDataBlob {
    fn drop(&mut self) {
        unsafe { aiReleaseExportBlob(self.raw) }
    }
}

extern {
    /// Returns the number of export file formats available in the current
    /// Assimp build.
    /// Use aiGetExportFormatDescription() to retrieve infos of a specific
    /// export format.
    pub fn aiGetExportFormatCount() -> size_t;

    /// Returns a description of the nth export file format.
    ///
    /// Use #aiGetExportFormatCount() to learn how many export formats are
    /// supported.
    ///
    /// #Parameters
    ///
    /// * `index` Index of the export format to retrieve information for.  Valid
    ///   range is 0 to #aiGetExportFormatCount()
    ///
    /// Return a description of that specific export format. NULL if index
    /// is out of range.
    pub fn aiGetExportFormatDescription(index: size_t) -> *const ExportFormatDesc;

    ///  Create a modifyable copy of a scene.
    ///
    ///  This is useful to import files via Assimp, change their topology and
    ///  export them again. Since the scene returned by the various importer
    ///  functions is const, a modifyable copy is needed.
    ///
    /// #Parameters
    ///
    /// * `in` Valid scene to be copied
    /// * `out` Receives a modifyable copy of the scene.
    pub fn aiCopyScene(input: *const RawScene, output: *mut*mut RawScene);

    /// Exports the given scene to a chosen file format and writes the result
    /// file(s) to disk.
    ///
    /// * pScene The scene to export. Stays in possession of the caller,
    ///        is not changed by the function.
    ///
    ///  The scene is expected to conform to Assimp's Importer output format
    ///  as specified in the @link data Data Structures Page @endlink. In
    ///  short, this means the model data should use a right-handed coordinate
    ///  systems, face winding should be counter-clockwise and the UV
    ///  coordinate origin is assumed to be in the upper left. If your input
    ///  data uses different conventions, have a look at the last parameter.
    ///
    /// @param pFormatId ID string to specify to which format you want to
    ///     export to. Use aiGetExportFormatCount() /
    ///     aiGetExportFormatDescription() to learn which export formats are
    ///     available.
    ///
    /// @param pFileName Output file to write
    ///
    /// @param pIO custom IO implementation to be used. Use this if you use
    ///     your own storage methods.  If none is supplied, a default
    ///     implementation using standard file IO is used. Note that
    ///     #aiExportSceneToBlob is provided as convenience function to export
    ///     to memory buffers.
    ///
    /// @param pPreprocessing Accepts any choice of the #aiPostProcessing
    ///      enumerated flags, but in reality only a subset of them makes
    ///      sense here. Specifying 'preprocessing' flags is useful if the
    ///      input scene does not conform to Assimp's default conventions as
    ///      specified in the @link data Data Structures Page @endlink.  In
    ///      short, this means the geometry data should use a right-handed
    ///      coordinate systems, face winding should be counter-clockwise and
    ///      the UV coordinate origin is assumed to be in the upper left. The
    ///      #aiProcess_MakeLeftHanded, #aiProcess_FlipUVs and
    ///      #aiProcess_FlipWindingOrder flags are used in the import side to
    ///      allow users to have those defaults automatically adapted to their
    ///      conventions. Specifying those flags for exporting has the
    ///      opposite effect, respectively. Some other of the
    ///      #aiPostProcessSteps enumerated values may be useful as well, but
    ///      you'll need to try out what their effect on the exported file is.
    ///      Many formats impose their own restrictions on the structure of
    ///      the geometry stored therein, so some preprocessing may have
    ///      little or no effect at all, or may be redundant as exporters
    ///      would apply them anyhow. A good example is triangulation - whilst
    ///      you can enforce it by specifying the #aiProcess_Triangulate flag,
    ///      most export formats support only triangulate data so they would
    ///      run the step anyway.
    ///
    /// Returns a status code indicating the result of the export
    pub fn aiExportScene( scene: *const RawScene,
                      formatId: *const c_char,
                      file_name: *const c_char,
                      preprocessing: c_uint)
                      -> Return;


    /// Releases the memory associated with the given exported data. Use this function to free a data blob
    /// returned by aiExportScene().
    ///
    /// data the data blob returned by #aiExportSceneToBlob
    pub fn aiReleaseExportBlob(data: *const RawExportDataBlob);


    /// Exports the given scene to a chosen file format using custom IO logic supplied by you.
    /// * pScene The scene to export. Stays in possession of the caller, is not changed by the function.
    /// * pFormatId ID string to specify to which format you want to export to. Use
    ///aiGetExportFormatCount() / aiGetExportFormatDescription() to learn which export formats are available.
    /// * pFileName Output file to write
    /// * pIO custom IO implementation to be used. Use this if you use your own storage methods.
    ///  If none is supplied, a default implementation using standard file IO is used. Note that
    ///  #aiExportSceneToBlob is provided as convenience function to export to memory buffers.
    /// * pPreprocessing Please see the documentation for #aiExportScene
    ///
    /// Returns a status code indicating the result of the export
    pub fn aiExportSceneEx( scene: *const RawScene,
                        format_id: *const c_char,
                        file_name: *const c_char,
                        file_io: *mut AiFileIO,
                        preprocessing: c_uint )
                        -> Return;

    ///  Exports the given scene to a chosen file format.
    ///
    ///  Returns the exported data as a binary blob which you can write into a
    ///  file or something.  When you're done with the data, use
    ///  #aiReleaseExportBlob() to free the resources associated with the
    ///  export.  @param pScene The scene to export. Stays in possession of
    ///  the caller, is not changed by the function.  @param pFormatId ID
    ///  string to specify to which format you want to export to. Use
    ///  #aiGetExportFormatCount() / #aiGetExportFormatDescription() to learn
    ///  which export formats are available.  @param pPreprocessing Please see
    ///  the documentation for #aiExportScene @return the exported data or
    ///  NULL in case of error
    pub fn aiExportSceneToBlob( scene: *const RawScene,
                            format_id: *const c_char,
                            preprocessing: c_uint )
                            -> *const RawExportDataBlob;
}
