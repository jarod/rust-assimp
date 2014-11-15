
use libc::{c_char};
use std::ptr;
// use std::c_str::CString;

use ffi;

// typedef void (*aiLogStreamCallback)(const char* /* message */,
//                                     char* /* user */);
type LogStreamCallback = fn (*const c_char /* msg */, *mut c_char /* user */);

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

pub enum LogStreamType<'a> {
    LogStreamFile(&'a str),
    LogStreamStdout,
    LogStreamStderr,
    LogStreamDebugger,
    // LogStreamCustom(_)
}

/// C-API: Represents a log stream. A log stream receives all log messages and
/// streams them _somewhere_.
#[repr(C)]
pub struct LogStream {
    /// callback to be called
    callback: LogStreamCallback,

    /// user data to be passed to the callback
    user: *mut c_char,
}

// TODO
// pub struct Logger {
//     log: Vec<LogStream>
// }

// pub fn get_error_string() -> Option<String> {
//     unsafe {
//         let pstr = ffi::aiGetErrorString();
//         if pstr.is_null() {
//             return None;
//         }
//         let cstr = CString::new(pstr as *const i8, false);
//         match cstr {
//             Some(ss) => Some(ss.into_string()),
//             None => None,
//         }
//     }
// }

pub fn add_log_stream(stream: LogStreamType) {
    unsafe {
        let null = ptr::null();
        let log = match stream {
            LogStreamFile(fname) => fname.with_c_str(|s|
                ffi::aiGetPredefinedLogStream(DefaultLogStream_FILE, s) ),
            LogStreamStdout =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_STDOUT, null),
            LogStreamStderr =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_STDERR, null),
            LogStreamDebugger =>
                ffi::aiGetPredefinedLogStream(DefaultLogStream_DEBUGGER, null),
        };
        ffi::aiAttachLogStream(&log);
    }
}

pub fn detach_all_log_streams() {
    unsafe {
        ffi::aiDetachAllLogStreams();
    }
}
