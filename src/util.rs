use std::raw;
use std::mem;

#[inline(always)]
pub unsafe fn ptr_ptr_to_slice<'a, T>(ptr: *mut*mut T, len: uint) -> &'a [&'a T] {
    let raw_slice : raw::Slice<&T> = raw::Slice {
        data: mem::transmute(ptr),
        len: len,
    };
    mem::transmute(raw_slice)
}

#[inline(always)]
pub unsafe fn ptr_to_slice<'a, T>(ptr: *mut T, len: uint) -> &'a [T] {
    let raw_slice : raw::Slice<T> = raw::Slice {
        data: mem::transmute(ptr),
        len: len,
    };
    mem::transmute(raw_slice)
}

///	Standard return type for some library functions.
#[repr(C)]
pub enum Return {
    /// Indicates that a function was successful 
    Return_SUCCESS = 0x0,

    /// Indicates that a function failed 
    Return_FAILURE = -0x1,

    /// Indicates that not enough memory was availabe to perform the requested 
    /// operation
    Return_OUTOFMEMORY = -0x3,
}

/// Seek origins (for the virtual file system API).
///
#[repr(C)]
pub enum Origin {
    /// Beginning of the file 
    Origin_SET = 0x0,

    /// Current position of the file pointer 
    Origin_CUR = 0x1,

    /// End of the file, offsets must be negative 
    Origin_END = 0x2,
}

/// Enumerates predefined log streaming destinations.
///
/// Logging to these streams can be enabled with a single call to
///  #LogStream::createDefaultStream or #aiAttachPredefinedLogStream(),
///  respectively.
pub enum aiDefaultLogStream {
    /// Stream the log to a file 
    aiDefaultLogStream_FILE = 0x1,

    /// Stream the log to std::cout 
    aiDefaultLogStream_STDOUT = 0x2,

    /// Stream the log to std::cerr 
    aiDefaultLogStream_STDERR = 0x4,

    /// MSVC only: Stream the log the the debugger
    /// (this relies on OutputDebugString from the Win32 SDK)
    aiDefaultLogStream_DEBUGGER = 0x8,
}
