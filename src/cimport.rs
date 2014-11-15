
use scene::{RawScene};
use libc::{c_char, c_uint, c_float, c_int};
use types::{AiBool, AiString, MemoryInfo};


/// C-API: Represents an opaque set of settings to be used during importing.
///
/// * see aiCreatePropertyStore
/// * see aiReleasePropertyStore
/// * see aiImportFileExWithProperties
/// * see aiSetPropertyInteger
/// * see aiSetPropertyFloat
/// * see aiSetPropertyString
#[repr(C)]
pub struct PropertyStore {
    sentinel: c_char,
}

#[link(name = "assimp")]
extern {
    ///  Reads the given file and returns its content.
    ///
    /// If the call succeeds, the imported data is returned in an aiScene structure.
    /// The data is intended to be read-only, it stays property of the ASSIMP
    /// library and will be stable until aiReleaseImport() is called. After you're
    /// done with it, call aiReleaseImport() to free the resources associated with
    /// this file. If the import fails, NULL is returned instead. Call
    /// aiGetErrorString() to retrieve a human-readable error text.
    /// @param pFile Path and filename of the file to be imported,
    ///   expected to be a null-terminated c-string. NULL is not a valid value.
    /// @param pFlags Optional post processing steps to be executed after
    ///   a successful import. Provide a bitwise combination of the
    ///   aiPostProcessSteps flags.
    /// @return Pointer to the imported data or NULL if the import failed.
    ///
    pub fn aiImportFile(fname: *const c_char, flags: c_uint) -> *const RawScene;

    /// Returns the error text of the last failed import process.
    ///
    /// @return A textual description of the error that occurred at the last
    /// import process. NULL if there was no error. There can't be an error if you
    /// got a non-NULL aiScene from aiImportFile/aiImportFileEx/aiApplyPostProcessing.
    ///
    pub fn aiGetErrorString() -> *const c_char;


    /// Reads the given file from a given memory buffer,
    ///
    /// If the call succeeds, the contents of the file are returned as a
    /// pointer to an aiScene object. The returned data is intended to be
    /// read-only, the importer keeps ownership of the data and will destroy
    /// it upon destruction. If the import fails, NULL is returned.  A
    /// human-readable error description can be retrieved by calling
    /// aiGetErrorString().
    /// # Arguments
    ///
    /// * `buffer` Pointer to the file data
    /// * `length` Length of pBuffer, in bytes
    /// * `flags` Optional post processing steps to be executed after
    ///   a successful import. Provide a bitwise combination of the
    ///   aiPostProcessSteps flags. If you wish to inspect the imported
    ///   scene first in order to fine-tune your post-processing setup,
    ///   consider to use aiApplyPostProcessing().
    /// * `hint` An additional hint to the library. If this is a non empty
    ///   string, the library looks for a loader to support the file extension
    ///   specified by pHint and passes the file to the first matching loader.
    ///   If this loader is unable to completely the request, the library
    ///   continues and tries to determine the file format on its own, a task
    ///   that may or may not be successful.  Check the return value, and
    ///   you'll know ...
    ///
    /// A pointer to the imported data, NULL if the import failed.
    ///
    /// Note: This is a straightforward way to decode models from memory
    /// buffers, but it doesn't handle model formats spreading their data
    /// across multiple files or even directories. Examples include OBJ or
    /// MD3, which outsource parts of their material stuff into external
    /// scripts. If you need the full functionality, provide a custom IOSystem
    /// to make Assimp find these files.
    pub fn aiImportFileFromMemory(buf: *const c_char,
                              len: c_uint,
                              flags: c_uint,
                              hint: *const c_char)
                              ->  *const RawScene;

    /// Same as aiImportFileFromMemory, but adds an extra parameter
    /// containing importer settings.
    ///
    /// * props PropertyStore instance containing import settings.
    pub fn aiImportFileFromMemoryWithProperties(buf: *const c_char,
                                            len: c_uint,
                                            flags: c_uint,
                                            hint: *const c_char,
                                            props: *const PropertyStore)
                                            ->  *const RawScene;

    /// Apply post-processing to an already-imported scene.
    ///
    /// This is strictly equivalent to calling aiImportFile()/aiImportFileEx
    /// with the same flags. However, you can use this separate function to
    /// inspect the imported scene first to fine-tune your post-processing
    /// setup.
    ///
    /// # Parameters
    ///
    /// * `scene` Scene to work on.
    /// * `flags` Provide a bitwise combination of the aiPostProcessSteps flags.
    ///
    /// Returns a pointer to the post-processed data.
    ///
    /// Post processing is done in-place, meaning this is still the same
    /// aiScene which you passed for pScene. However, _if_ post-processing
    /// failed, the scene could now be NULL. That's quite a rare case, post
    /// processing steps are not really designed to 'fail'. To be exact, the
    /// aiProcess_ValidateDS flag is currently the only post processing step
    /// which can actually cause the scene to be reset to NULL.
    pub fn aiApplyPostProcessing(scene: *mut RawScene,
                             flags: c_uint)
                             -> *const RawScene;

    /// Enable verbose logging.
    ///
    /// Verbose logging includes debug-related stuff
    /// and detailed import statistics. This can have severe impact on import
    /// performance and memory consumption. However, it might be useful to
    /// find out why a file didn't read correctly.
    pub fn aiEnableVerboseLogging(enable: AiBool);

    /// Releases all resources associated with the given import process.
    ///
    /// Call this function after you're done with the imported data.
    /// pScene The imported data to release. NULL is a valid value.
    pub fn aiReleaseImport(scene: *const RawScene);

    /// Get the approximated storage required by an imported asset
    ///
    /// # Parameters
    ///
    /// * pIn Input asset.
    /// * in Data structure to be filled.
    pub fn aiGetMemoryRequirements(scene: *const RawScene, info: *mut MemoryInfo);

    /// Create an empty property store.
    ///
    /// Property stores are used to collect import settings.
    /// Returns a new property store. Property stores need to
    /// be manually destroyed using the aiReleasePropertyStore API function.
    pub fn aiCreatePropertyStore() -> *mut PropertyStore;

    /// Delete a property store.
    pub fn aiReleasePropertyStore(p: *mut PropertyStore);

    /// Set an integer property.
    ///
    /// This is the C-version of Assimp::Importer::SetPropertyInteger(). In
    /// the C interface, properties are always shared by all imports. It is
    /// not possible to specify them per import.
    ///
    /// * `name` Name of the configuration property to be set. All supported
    ///   public properties are defined in the config.h header file (AI_CONFIG_XXX).
    /// * `value` New value for the property
    pub fn aiSetImportPropertyInteger(store: *mut PropertyStore,
                                      name: *const c_char,
                                      value: c_int);

    /// Set a floating-point property.
    ///
    /// This is the C-version of Assimp::Importer::SetPropertyFloat(). In the
    /// C interface, properties are always shared by all imports. It is not
    /// possible to specify them per import.
    ///
    /// `name` Name of the configuration property to be set. All supported
    ///        public properties are defined in the config.h header file
    /// `value` New value for the property
    ///
    pub fn aiSetImportPropertyFloat(store: *mut PropertyStore,
                                    name: *const c_char,
                                    value: c_float);

    /// Set a string property.
    ///
    /// This is the C-version of Assimp::Importer::SetPropertyString(). In
    /// the C interface, properties are always shared by all imports. It is
    /// not possible to specify them per import.
    ///
    /// # Parameters
    /// * property store to modify. Use aiCreatePropertyStore to obtain a store.
    /// * szName Name of the configuration property to be set. All supported
    ///   public properties are defined in the config.h header file
    ///   (AI_CONFIG_XXX).
    /// * value New value for the property
    ///
    pub fn aiSetImportPropertyString(store: *mut PropertyStore,
                                     name: *const c_char,
                                     st: *const AiString);
}

// typedef void (*aiLogStreamCallback)(const char* /* message */, char* /* user */);

// /** C-API: Represents a log stream. A log stream receives all log messages and
//     *  streams them _somewhere_.
//     *  @see aiGetPredefinedLogStream
//     *  @see aiAttachLogStream
//     *  @see aiDetachLogStream */
// struct LogStream {
//     /** callback to be called */
//     aiLogStreamCallback callback;

//     /** user data to be passed to the callback */
//     char* user;
// };

// /** Reads the given file using user-defined I/O functions and returns
//     *   its content.
//     *
//     * If the call succeeds, the imported data is returned in an aiScene structure.
//     * The data is intended to be read-only, it stays property of the ASSIMP
//     * library and will be stable until aiReleaseImport() is called. After you're
//     * done with it, call aiReleaseImport() to free the resources associated with
//     * this file. If the import fails, NULL is returned instead. Call
//     * aiGetErrorString() to retrieve a human-readable error text.
//     * @param pFile Path and filename of the file to be imported,
//     *   expected to be a null-terminated c-string. NULL is not a valid value.
//     * @param pFlags Optional post processing steps to be executed after
//     *   a successful import. Provide a bitwise combination of the
//     *   #aiPostProcessSteps flags.
//     * @param pFS aiFileIO structure. Will be used to open the model file itself
//     *   and any other files the loader needs to open.  Pass NULL to use the default
//     *   implementation.
//     * @return Pointer to the imported data or NULL if the import failed.
//     * @note Include <aiFileIO.h> for the definition of #aiFileIO.
//     */
// ASSIMP_API const C_STRUCT aiScene* aiImportFileEx(
//     const char* pFile,
//     unsigned int pFlags,
//     C_STRUCT aiFileIO* pFS);

// /** Same as #aiImportFileEx, but adds an extra parameter containing importer settings.
//     *
//     * @param pProps #aiPropertyStore instance containing import settings.
//     * @see aiImportFileEx
//     */
// ASSIMP_API const C_STRUCT aiScene* aiImportFileExWithProperties(
//     const char* pFile,
//     unsigned int pFlags,
//     C_STRUCT aiFileIO* pFS,
//     const C_STRUCT aiPropertyStore* pProps);

// /** Get one of the predefine log streams. This is the quick'n'easy solution to//{{{
//     *  access Assimp's log system. Attaching a log stream can slightly reduce Assimp's
//     *  overall import performance.
//     *
//     *  Usage is rather simple (this will stream the log to a file, named log.txt, and
//     *  the stdout stream of the process:
//     *  @code
//     *    struct aiLogStream c;
//     *    c = aiGetPredefinedLogStream(aiDefaultLogStream_FILE,"log.txt");
//     *    aiAttachLogStream(&c);
//     *    c = aiGetPredefinedLogStream(aiDefaultLogStream_STDOUT,NULL);
//     *    aiAttachLogStream(&c);
//     *  @endcode
//     *
//     *  @param pStreams One of the #aiDefaultLogStream enumerated values.
//     *  @param file Solely for the #aiDefaultLogStream_FILE flag: specifies the file to write to.
//     *    Pass NULL for all other flags.
//     *  @return The log stream. callback is set to NULL if something went wrong.
//     */
// ASSIMP_API C_STRUCT aiLogStream aiGetPredefinedLogStream(
//     C_ENUM aiDefaultLogStream pStreams,
//     const char* file);//}}}

// /** Attach a custom log stream to the libraries' logging system.//{{{
//     *
//     *  Attaching a log stream can slightly reduce Assimp's overall import
//     *  performance. Multiple log-streams can be attached.
//     *  @param stream Describes the new log stream.
//     *  @note To ensure proepr destruction of the logging system, you need to manually
//     *    call aiDetachLogStream() on every single log stream you attach.
//     *    Alternatively (for the lazy folks) #aiDetachAllLogStreams is provided.
//     */
// ASSIMP_API void aiAttachLogStream(
//     const C_STRUCT aiLogStream* stream);//}}}

// /** Detach a custom log stream from the libraries' logging system.//{{{
//     *
//     *  This is the counterpart of #aiAttachPredefinedLogStream. If you attached a stream,
//     *  don't forget to detach it again.
//     *  @param stream The log stream to be detached.
//     *  @return AI_SUCCESS if the log stream has been detached successfully.
//     *  @see aiDetachAllLogStreams
//     */
// ASSIMP_API C_ENUM aiReturn aiDetachLogStream(
//     const C_STRUCT aiLogStream* stream);//}}}

// /** Detach all active log streams from the libraries' logging system.//{{{
//     *  This ensures that the logging system is terminated properly and all
//     *  resources allocated by it are actually freed. If you attached a stream,
//     *  don't forget to detach it again.
//     *  @see aiAttachLogStream
//     *  @see aiDetachLogStream
//     */
// ASSIMP_API void aiDetachAllLogStreams(void);//}}}

