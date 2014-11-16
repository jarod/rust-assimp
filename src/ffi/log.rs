//! Data types used in the assimp C api for logging
use libc::{c_char};

use types::{AiBool};

/// Callback function used for custom log stream
pub type LogStreamCallback = extern fn (*const c_char /* msg */, *mut c_char /* user */);

/// Enumerates predefined log streaming destinations.
///
/// Logging to these streams can be enabled with a single call to
///  #LogStream::createDefaultStream or #aiAttachPredefinedLogStream(),
///  respectively.
#[repr(C)]
pub enum DefaultLogStream {
    /// Stream the log to a file
    DefaultLogStream_FILE = 0x1,

    /// Stream the log to std::cout
    DefaultLogStream_STDOUT = 0x2,

    /// Stream the log to std::cerr
    DefaultLogStream_STDERR = 0x4,

    /// MSVC only: Stream the log the the debugger
    /// (thees relies on OutputDebugString from the Win32 SDK)
    DefaultLogStream_DEBUGGER = 0x8,
}

/// C-API: Represents a log stream. A log stream receives all log messages and
/// streams them _somewhere_.
#[repr(C)]
pub struct LogStream {
    /// callback to be called
    pub callback: LogStreamCallback,

    /// user data to be passed to the callback
    pub user: *mut c_char,
}

extern {
    /// Enable verbose logging.
    ///
    /// Verbose logging includes debug-related stuff
    /// and detailed import statistics. This can have severe impact on import
    /// performance and memory consumption. However, it might be useful to
    /// find out why a file didn't read correctly.
    pub fn aiEnableVerboseLogging(enable: AiBool);

    /// Get one of the predefine log streams.
    ///
    /// This is the quick'n'easy solution to access Assimp's log system.
    /// Attaching a log stream can slightly reduce Assimp's overall import
    /// performance.
    ///
    /// Usage is rather simple. This example will stream the log to a file,
    /// named log.txt, and the stdout stream of the process:
    ///
    /// ```c
    ///   struct aiLogStream c;
    ///   c = aiGetPredefinedLogStream(aiDefaultLogStream_FILE,"log.txt");
    ///   aiAttachLogStream(&c);
    ///   c = aiGetPredefinedLogStream(aiDefaultLogStream_STDOUT,NULL);
    ///   aiAttachLogStream(&c);
    /// ```
    ///
    /// # Parameters
    ///
    /// * pStreams One of the #aiDefaultLogStream enumerated values.
    ///
    /// * `file` Solely for the #aiDefaultLogStream_FILE flag: specifies the
    ///   file to write to.  Pass NULL for all other flags.
    ///
    /// The log stream. callback is set to NULL if something went wrong.
    // ASSIMP_API C_STRUCT aiLogStream aiGetPredefinedLogStream(
    //     C_ENUM aiDefaultLogStream pStreams,
    //     const char* file);
    pub fn aiGetPredefinedLogStream(stream: DefaultLogStream,
                                file: *const c_char)
                                -> LogStream;

    /// Attach a custom log stream to the libraries' logging system.
    ///
    /// Attaching a log stream can slightly reduce Assimp's overall import
    /// performance. Multiple log-streams can be attached.
    /// @param stream Describes the new log stream.
    /// @note To ensure proepr destruction of the logging system, you need to manually
    ///   call aiDetachLogStream() on every single log stream you attach.
    ///   Alternatively (for the lazy folks) #aiDetachAllLogStreams is provided.
    ///
    // ASSIMP_API void aiAttachLogStream( const C_STRUCT aiLogStream* stream);
    pub fn aiAttachLogStream(stream: *const LogStream);

    /// Detach a custom log stream from the libraries' logging system.
    ///
    /// This is the counterpart of #aiAttachPredefinedLogStream.
    /// If you attached a stream, don't forget to detach it again.
    ///
    /// @param stream The log stream to be detached.
    /// @return AI_SUCCESS if the log stream has been detached successfully.
    /// @see aiDetachAllLogStreams
    // ASSIMP_API C_ENUM aiReturn aiDetachLogStream(
    // const C_STRUCT aiLogStream* stream);

    /// Detach all active log streams from the libraries' logging system.
    ///
    /// This ensures that the logging system is terminated properly and all
    /// resources allocated by it are actually freed. If you attached a stream,
    /// don't forget to detach it again.
    // ASSIMP_API void aiDetachAllLogStreams(void);
    pub fn aiDetachAllLogStreams();
}
