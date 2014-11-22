/// Defines generic C routines to access memory-mapped files

use libc::{c_char, size_t};

use types::{Return};

// AiFile callbacks
type FileWriteProc = extern fn (*mut AiFile, *const char, size_t, size_t) -> size_t;
type FileReadProc = extern fn (*mut AiFile, *const char, size_t, size_t) -> size_t;
type FileTellProc = extern fn (*mut AiFile) -> size_t;
type FileFlushProc = extern fn (*mut AiFile);
type FileSeek = extern fn (*mut AiFile, size_t, Origin) -> Return;

// AiFileIO callbacks
type FileOpenProc = extern fn (*mut AiFileIO, *const c_char, *const c_char) -> *mut AiFile;
type FileCloseProc = extern fn (*mut AiFileIO, *mut AiFile);

/// Seek origins (for the virtual file system API).
///
#[repr(C)]
#[allow(dead_code)]
pub enum Origin {
    /// Beginning of the file 
    Origin_SET = 0x0,

    /// Current position of the file pointer 
    Origin_CUR = 0x1,

    /// End of the file, offsets must be negative 
    Origin_END = 0x2,
}

/// C-API: File system callbacks
///
/// Provided are functions to open and close files. Supply a custom structure
/// to the import function. If you don't, a default implementation is used.
/// Use custom file systems to enable reading from other sources, such as ZIPs
/// or memory locations.
#[repr(C)]
#[allow(dead_code)]
pub struct AiFileIO {
    /// Function used to open a new file
    open: FileOpenProc,

    /// Function used to close an existing file
    close: FileCloseProc,

    /// User-defined, opaque data
    user_data: *const c_char,
}

/// File callbacks
///
/// Actually, it's a data structure to wrap a set of fXXXX (e.g fopen)
/// replacement functions.
///
/// The default implementation of the functions utilizes the fXXX functions from
/// the CRT. However, you can supply a custom implementation to Assimp by
/// delivering a custom aiFileIO. Use this to enable reading from other sources,
/// such as ZIP archives or memory locations.
#[repr(C)]
pub struct AiFile {
    /// Callback to read from a file
    read: FileReadProc,

    /// Callback to write to a file
    write: FileWriteProc,

    /// Callback to retrieve the current position of the file cursor (ftell())
    tell: FileTellProc,

    /// Callback to retrieve the size of the file, in bytes
    size: FileTellProc,

    /// Callback to set the current position of the file cursor (fseek())
    seek: FileSeek,

    /// Callback to flush the file contents
    flush: FileFlushProc,

    /// User-defined, opaque data
    user_data: *const c_char,
}
// vim: et tw=78 sw=4:
